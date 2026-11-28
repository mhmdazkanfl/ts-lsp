use zed_extension_api as z;

struct NativePackage {
    name: String,
    binary: std::path::PathBuf,
    manifest: std::path::PathBuf,
}

pub(super) trait Npm {
    fn latest_version(&mut self, name: &str) -> z::Result<String>;
    fn install(&mut self, name: &str, version: &str) -> z::Result<()>;
}

impl NativePackage {
    fn for_platform(
        work_dir: &std::path::Path,
        os: z::Os,
        arch: z::Architecture,
    ) -> z::Result<Self> {
        let (platform, executable) = match os {
            z::Os::Linux => ("linux", "tsc"),
            z::Os::Mac => ("darwin", "tsc"),
            z::Os::Windows => ("win32", "tsc.exe"),
        };
        let arch = match arch {
            z::Architecture::Aarch64 => "arm64",
            z::Architecture::X8664 => "x64",
            z::Architecture::X86 => {
                return Err("TypeScript 7 requires a 64-bit platform".into());
            }
        };
        let name = format!("@typescript/typescript-{platform}-{arch}");
        let directory = work_dir
            .join("node_modules")
            .join(&name);
        Ok(Self {
            name,
            binary: directory.join("lib").join(executable),
            manifest: directory.join("package.json"),
        })
    }

    fn installed_release(&self) -> Option<super::release::Release> {
        let binary = std::fs::metadata(&self.binary).ok()?;
        if !binary.is_file() || binary.len() == 0 {
            return None;
        }

        let manifest = std::fs::read_to_string(&self.manifest).ok()?;
        let package: z::serde_json::Value = z::serde_json::from_str(&manifest).ok()?;
        if package.get("name")?.as_str()? != self.name {
            return None;
        }
        super::release::Release::parse(package.get("version")?.as_str()?).ok()
    }

    fn latest_release(&self, npm: &mut impl Npm) -> z::Result<super::release::Release> {
        let name = &self.name;
        let latest = npm
            .latest_version(name)
            .map_err(|error| format!("Failed to check the latest version of {name}: {error}"))?;
        super::release::Release::parse(&latest).map_err(|error| error.for_package(name))
    }

    fn install(&self, npm: &mut impl Npm, release: &super::release::Release) -> z::Result<()> {
        let name = &self.name;
        npm.install(name, &release.to_string())
            .map_err(|error| format!("Failed to install {name}@{release}: {error}"))?;
        (self.installed_release().as_ref() == Some(release)).ok_or_else(|| {
            format!(
                "npm reported success for {name}@{release}, but the expected install at {} is missing or incomplete",
                self.binary.display()
            )
        })
    }
}

pub(super) fn prepare_server(
    work_dir: &std::path::Path,
    (os, arch): (z::Os, z::Architecture),
    npm: &mut impl Npm,
    mut report: impl FnMut(z::LanguageServerInstallationStatus),
) -> z::Result<std::path::PathBuf> {
    let package = NativePackage::for_platform(work_dir, os, arch)?;
    let cached = package.installed_release();
    report(z::LanguageServerInstallationStatus::CheckingForUpdate);

    let update = package
        .latest_release(npm)
        .and_then(|available| {
            if available.requires_download(cached.as_ref()) {
                report(z::LanguageServerInstallationStatus::Downloading);
                package.install(npm, &available)
            } else {
                Ok(())
            }
        });

    if let Err(error) = update {
        // npm may have changed the files before returning an error.
        let can_reuse_cache =
            cached.is_some_and(|original| package.installed_release() == Some(original));
        if !can_reuse_cache {
            report(z::LanguageServerInstallationStatus::Failed(error.clone()));
            return Err(error);
        }
        eprintln!("Using the installed TypeScript language server after an update failed: {error}");
    }

    report(z::LanguageServerInstallationStatus::None);
    Ok(package.binary)
}

#[cfg(test)]
mod tests {
    use zed_extension_api as z;

    macro_rules! selects_package {
        ($test:ident, $os:ident, $arch:ident, $name:literal, $binary:literal) => {
            #[test]
            fn $test() {
                let work_dir = std::path::Path::new("extension");

                let package = super::NativePackage::for_platform(
                    work_dir,
                    z::Os::$os,
                    z::Architecture::$arch,
                )
                .unwrap();

                assert_eq!(package.name, $name);
                assert_eq!(package.binary, work_dir.join($binary));
            }
        };
    }

    selects_package!(
        linux_selects_the_x64_package,
        Linux,
        X8664,
        "@typescript/typescript-linux-x64",
        "node_modules/@typescript/typescript-linux-x64/lib/tsc"
    );
    selects_package!(
        linux_selects_the_arm64_package,
        Linux,
        Aarch64,
        "@typescript/typescript-linux-arm64",
        "node_modules/@typescript/typescript-linux-arm64/lib/tsc"
    );
    selects_package!(
        intel_macs_select_the_x64_package,
        Mac,
        X8664,
        "@typescript/typescript-darwin-x64",
        "node_modules/@typescript/typescript-darwin-x64/lib/tsc"
    );
    selects_package!(
        apple_silicon_macs_select_the_arm64_package,
        Mac,
        Aarch64,
        "@typescript/typescript-darwin-arm64",
        "node_modules/@typescript/typescript-darwin-arm64/lib/tsc"
    );
    selects_package!(
        windows_selects_the_x64_executable,
        Windows,
        X8664,
        "@typescript/typescript-win32-x64",
        "node_modules/@typescript/typescript-win32-x64/lib/tsc.exe"
    );
    selects_package!(
        windows_selects_the_arm64_executable,
        Windows,
        Aarch64,
        "@typescript/typescript-win32-arm64",
        "node_modules/@typescript/typescript-win32-arm64/lib/tsc.exe"
    );
}
