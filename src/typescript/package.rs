use zed_extension_api as z;

pub trait Npm {
    fn latest_version(&self, name: &str) -> z::Result<String>;
    fn install(&self, name: &str, version: &str) -> z::Result<()>;
}

pub struct ZedNpm;

impl Npm for ZedNpm {
    fn latest_version(&self, name: &str) -> z::Result<String> {
        z::npm_package_latest_version(name)
    }

    fn install(&self, name: &str, version: &str) -> z::Result<()> {
        z::npm_install_package(name, version)
    }
}

pub struct PlatformPackage {
    name: String,
    manifest: std::path::PathBuf,
    binary: std::path::PathBuf,
}

impl PlatformPackage {
    pub fn new(work_dir: &std::path::Path, os: z::Os, arch: z::Architecture) -> z::Result<Self> {
        let (platform, executable) = match os {
            z::Os::Linux => ("linux", "tsc"),
            z::Os::Mac => ("darwin", "tsc"),
            z::Os::Windows => ("win32", "tsc.exe"),
        };
        let arch = match arch {
            z::Architecture::Aarch64 => "arm64",
            z::Architecture::X8664 => "x64",
            z::Architecture::X86 => return Err("TypeScript 7 requires a 64-bit platform".into()),
        };
        let name = format!("@typescript/typescript-{platform}-{arch}");
        let directory = work_dir
            .join("node_modules")
            .join(&name);
        Ok(Self {
            name,
            manifest: directory.join("package.json"),
            binary: directory.join("lib").join(executable),
        })
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn into_binary(self) -> std::path::PathBuf {
        self.binary
    }

    pub fn installed_version(&self) -> Option<semver::Version> {
        if !std::fs::metadata(&self.binary).is_ok_and(|file| file.is_file() && file.len() > 0) {
            return None;
        }

        // A cached binary can start without initializing npm.
        let manifest = std::fs::read_to_string(&self.manifest).ok()?;
        let package: z::serde_json::Value = z::serde_json::from_str(&manifest).ok()?;
        if package.get("name")?.as_str()? != self.name {
            return None;
        }
        semver::Version::parse(package.get("version")?.as_str()?).ok()
    }

    pub fn latest_version(&self, npm: &impl Npm) -> z::Result<semver::Version> {
        let latest = npm
            .latest_version(&self.name)
            .map_err(|err| {
                format!(
                    "Failed to check the latest version of {}: {err}",
                    self.name
                )
            })?;
        semver::Version::parse(&latest).map_err(|err| {
            format!(
                "Invalid version {latest} for {}: {err}",
                self.name
            )
        })
    }

    pub fn install(&self, npm: &impl Npm, version: &semver::Version) -> z::Result<()> {
        npm.install(&self.name, &version.to_string())
            .map_err(|err| {
                format!(
                    "Failed to install {}@{version}: {err}",
                    self.name
                )
            })?;
        if self.installed_version().as_ref() != Some(version) {
            return Err(format!(
                "npm reported success for {}@{version}, but the expected install at {} is missing or incomplete",
                self.name,
                self.binary.display()
            ));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use zed_extension_api as z;

    #[test]
    fn maps_each_supported_platform_to_its_native_package() {
        let work_dir = std::path::Path::new("");
        for (os, platform, executable) in [
            (z::Os::Linux, "linux", "tsc"),
            (z::Os::Mac, "darwin", "tsc"),
            (z::Os::Windows, "win32", "tsc.exe"),
        ] {
            for (arch, suffix) in [
                (z::Architecture::Aarch64, "arm64"),
                (z::Architecture::X8664, "x64"),
            ] {
                let package = super::PlatformPackage::new(work_dir, os, arch).unwrap();
                let name = format!("@typescript/typescript-{platform}-{suffix}");
                assert_eq!(package.name(), name);
                let directory = std::path::Path::new("node_modules").join(name);
                assert_eq!(
                    package.manifest,
                    directory.join("package.json")
                );
                assert_eq!(
                    package.into_binary(),
                    directory.join("lib").join(executable)
                );
            }
        }
    }

    #[test]
    fn rejects_32_bit_x86() {
        let result = super::PlatformPackage::new(
            std::path::Path::new(""),
            z::Os::Windows,
            z::Architecture::X86,
        );
        assert!(result.is_err());
    }
}
