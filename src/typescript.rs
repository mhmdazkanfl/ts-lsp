use zed_extension_api as z;

const MINIMUM_MAJOR_VERSION: u64 = 7;

trait Npm {
    fn latest_version(&mut self, name: &str) -> z::Result<String>;
    fn install(&mut self, name: &str, version: &str) -> z::Result<()>;
}

struct NativePackage {
    name: String,
    manifest: std::path::PathBuf,
    binary: std::path::PathBuf,
}

impl NativePackage {
    fn new(work_dir: &std::path::Path, os: z::Os, arch: z::Architecture) -> z::Result<Self> {
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
            manifest: directory.join("package.json"),
            binary: directory.join("lib").join(executable),
        })
    }

    fn ensure_executable(
        &self,
        npm: &mut impl Npm,
        mut report: impl FnMut(z::LanguageServerInstallationStatus),
    ) -> z::Result<std::path::PathBuf> {
        let cached = self
            .installed_version()
            .filter(is_supported);
        report(z::LanguageServerInstallationStatus::CheckingForUpdate);
        let update = self
            .latest_supported_version(npm)
            .and_then(|available| {
                let needs_update = cached.as_ref().is_none_or(|installed| {
                    installed
                        .cmp_precedence(&available)
                        .is_lt()
                });
                if needs_update {
                    report(z::LanguageServerInstallationStatus::Downloading);
                    self.install(npm, &available)
                } else {
                    Ok(())
                }
            });

        if let Err(error) = update {
            if cached.is_none() || self.installed_version() != cached {
                report(z::LanguageServerInstallationStatus::Failed(error.clone()));
                return Err(error);
            }
            eprintln!(
                "Using the installed TypeScript language server after an update failed: {error}"
            );
        }

        report(z::LanguageServerInstallationStatus::None);
        Ok(self.binary.clone())
    }

    fn installed_version(&self) -> Option<semver::Version> {
        if !std::fs::metadata(&self.binary).is_ok_and(|file| file.is_file() && file.len() > 0) {
            return None;
        }

        let manifest = std::fs::read_to_string(&self.manifest).ok()?;
        let package: z::serde_json::Value = z::serde_json::from_str(&manifest).ok()?;
        if package.get("name")?.as_str()? != self.name {
            return None;
        }
        semver::Version::parse(package.get("version")?.as_str()?).ok()
    }

    fn latest_supported_version(&self, npm: &mut impl Npm) -> z::Result<semver::Version> {
        let latest = npm
            .latest_version(&self.name)
            .map_err(|error| {
                format!(
                    "Failed to check the latest version of {}: {error}",
                    self.name
                )
            })?;
        let version = semver::Version::parse(&latest).map_err(|error| {
            format!(
                "Invalid version {latest} for {}: {error}",
                self.name
            )
        })?;
        if !is_supported(&version) {
            return Err(format!(
                "{}@{version} is unsupported. Expected a stable TypeScript version {MINIMUM_MAJOR_VERSION} or newer",
                self.name
            ));
        }
        Ok(version)
    }

    fn install(&self, npm: &mut impl Npm, version: &semver::Version) -> z::Result<()> {
        npm.install(&self.name, &version.to_string())
            .map_err(|error| {
                format!(
                    "Failed to install {}@{version}: {error}",
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

struct ZedNpm;

impl Npm for ZedNpm {
    fn latest_version(&mut self, name: &str) -> z::Result<String> {
        z::npm_package_latest_version(name)
    }

    fn install(&mut self, name: &str, version: &str) -> z::Result<()> {
        z::npm_install_package(name, version)
    }
}

pub(super) fn executable(
    language_server_id: &z::LanguageServerId,
) -> z::Result<std::path::PathBuf> {
    let (os, arch) = z::current_platform();
    let package = NativePackage::new(std::path::Path::new(""), os, arch)?;
    package.ensure_executable(&mut ZedNpm, |status| {
        z::set_language_server_installation_status(language_server_id, &status);
    })
}

fn is_supported(version: &semver::Version) -> bool {
    version.major >= MINIMUM_MAJOR_VERSION && version.pre.is_empty()
}

#[cfg(test)]
mod tests {
    use zed_extension_api as z;

    const PACKAGE_NAME: &str = "@typescript/typescript-win32-x64";

    enum InstallOutcome {
        Complete,
        MissingBinary,
        WrongVersion,
        Fail,
        FailAfterRemovingBinary,
    }

    struct FakeNpm<'a> {
        work_dir: &'a std::path::Path,
        latest: z::Result<&'static str>,
        outcome: InstallOutcome,
        checks: usize,
        versions_requested: Vec<String>,
    }

    impl super::Npm for FakeNpm<'_> {
        fn latest_version(&mut self, name: &str) -> z::Result<String> {
            assert_eq!(name, PACKAGE_NAME);
            self.checks += 1;
            self.latest.clone().map(str::to_owned)
        }

        fn install(&mut self, name: &str, version: &str) -> z::Result<()> {
            assert_eq!(name, PACKAGE_NAME);
            self.versions_requested
                .push(version.to_owned());
            match self.outcome {
                InstallOutcome::Complete => write_install(self.work_dir, version, true),
                InstallOutcome::MissingBinary => write_install(self.work_dir, version, false),
                InstallOutcome::WrongVersion => write_install(self.work_dir, "7.0.1", true),
                InstallOutcome::Fail => return Err("npm failed".into()),
                InstallOutcome::FailAfterRemovingBinary => {
                    std::fs::remove_file(binary_path(self.work_dir)).unwrap();
                    return Err("npm failed".into());
                }
            }
            Ok(())
        }
    }

    struct Fixture {
        work_dir: tempfile::TempDir,
        package: super::NativePackage,
    }

    impl Fixture {
        fn new() -> Self {
            let work_dir = tempfile::tempdir().unwrap();
            let package = super::NativePackage::new(
                work_dir.path(),
                z::Os::Windows,
                z::Architecture::X8664,
            )
            .unwrap();
            Self {
                work_dir,
                package,
            }
        }

        fn npm(&self, latest: z::Result<&'static str>, outcome: InstallOutcome) -> FakeNpm<'_> {
            FakeNpm {
                work_dir: self.work_dir.path(),
                latest,
                outcome,
                checks: 0,
                versions_requested: Vec::new(),
            }
        }

        fn start(&self, npm: &mut FakeNpm<'_>) -> z::Result<std::path::PathBuf> {
            self.package
                .ensure_executable(npm, |_| {})
        }

        fn assert_start_fails(&self, npm: &mut FakeNpm<'_>) {
            let mut last_status = None;
            let result = self
                .package
                .ensure_executable(npm, |status| {
                    last_status = Some(status);
                });
            assert!(result.is_err());
            assert!(matches!(
                last_status,
                Some(z::LanguageServerInstallationStatus::Failed(_))
            ));
        }
    }

    fn binary_path(work_dir: &std::path::Path) -> std::path::PathBuf {
        work_dir
            .join("node_modules")
            .join(PACKAGE_NAME)
            .join("lib")
            .join("tsc.exe")
    }

    fn write_install(work_dir: &std::path::Path, version: &str, has_binary: bool) {
        let directory = work_dir
            .join("node_modules")
            .join(PACKAGE_NAME);
        std::fs::create_dir_all(directory.join("lib")).unwrap();
        let manifest = z::serde_json::json!({"name": PACKAGE_NAME, "version": version});
        std::fs::write(
            directory.join("package.json"),
            manifest.to_string(),
        )
        .unwrap();
        if has_binary {
            std::fs::write(
                binary_path(work_dir),
                b"native executable",
            )
            .unwrap();
        }
    }

    #[test]
    fn cold_start_installs_the_exact_version() {
        let fixture = Fixture::new();
        let mut npm = fixture.npm(Ok("7.0.2"), InstallOutcome::Complete);
        assert_eq!(
            fixture.start(&mut npm).unwrap(),
            binary_path(fixture.work_dir.path())
        );
        assert_eq!(npm.versions_requested, ["7.0.2"]);
    }

    #[test]
    fn cold_start_rejects_an_unsupported_release_without_downloading_it() {
        let fixture = Fixture::new();
        let mut npm = fixture.npm(
            Ok("7.0.0-rc.1"),
            InstallOutcome::Complete,
        );

        fixture.assert_start_fails(&mut npm);

        assert_eq!(
            npm.versions_requested,
            Vec::<String>::new()
        );
    }

    #[test]
    fn each_start_checks_npm_and_reuses_the_current_version() {
        let fixture = Fixture::new();
        write_install(fixture.work_dir.path(), "7.0.2", true);
        let mut npm = fixture.npm(Ok("7.0.2"), InstallOutcome::Fail);
        fixture.start(&mut npm).unwrap();
        fixture.start(&mut npm).unwrap();
        assert_eq!(npm.checks, 2);
        assert_eq!(
            npm.versions_requested,
            Vec::<String>::new()
        );
    }

    #[test]
    fn older_versions_and_build_metadata_changes_do_not_replace_a_cached_install() {
        let fixture = Fixture::new();
        write_install(fixture.work_dir.path(), "7.0.2", true);
        for available in ["7.0.1", "7.0.2+build"] {
            let mut npm = fixture.npm(Ok(available), InstallOutcome::Fail);
            fixture.start(&mut npm).unwrap();
            assert_eq!(
                npm.versions_requested,
                Vec::<String>::new()
            );
        }
    }

    #[test]
    fn a_failed_version_lookup_reuses_the_installed_binary() {
        let fixture = Fixture::new();
        write_install(fixture.work_dir.path(), "7.0.2", true);
        let mut npm = fixture.npm(
            Err("offline".into()),
            InstallOutcome::Fail,
        );
        fixture.start(&mut npm).unwrap();
        assert_eq!(npm.checks, 1);
        assert_eq!(
            npm.versions_requested,
            Vec::<String>::new()
        );
    }

    #[test]
    fn a_failed_version_lookup_does_not_reuse_an_unsupported_cached_binary() {
        let fixture = Fixture::new();
        write_install(
            fixture.work_dir.path(),
            "7.0.0-rc.1",
            true,
        );
        let mut npm = fixture.npm(
            Err("offline".into()),
            InstallOutcome::Fail,
        );

        fixture.assert_start_fails(&mut npm);
    }

    #[test]
    fn npm_success_requires_an_executable() {
        let fixture = Fixture::new();
        let mut npm = fixture.npm(
            Ok("7.0.2"),
            InstallOutcome::MissingBinary,
        );
        fixture.assert_start_fails(&mut npm);
        assert!(
            fixture
                .package
                .installed_version()
                .is_none()
        );
    }

    #[test]
    fn a_successful_update_installs_the_new_version() {
        let fixture = Fixture::new();
        write_install(fixture.work_dir.path(), "7.0.2", true);
        let mut npm = fixture.npm(Ok("7.0.3"), InstallOutcome::Complete);
        fixture.start(&mut npm).unwrap();
        assert_eq!(
            fixture.package.installed_version(),
            Some(semver::Version::parse("7.0.3").unwrap())
        );
    }

    #[test]
    fn npm_success_requires_the_expected_version() {
        let fixture = Fixture::new();
        let mut npm = fixture.npm(
            Ok("7.0.2"),
            InstallOutcome::WrongVersion,
        );
        fixture.assert_start_fails(&mut npm);
        assert_eq!(
            fixture.package.installed_version(),
            Some(semver::Version::parse("7.0.1").unwrap())
        );
    }

    #[test]
    fn a_failed_update_reuses_the_previous_install_if_it_is_intact() {
        let fixture = Fixture::new();
        write_install(fixture.work_dir.path(), "7.0.2", true);
        let mut npm = fixture.npm(Ok("7.0.3"), InstallOutcome::Fail);
        fixture.start(&mut npm).unwrap();
        assert_eq!(npm.versions_requested, ["7.0.3"]);
        assert_eq!(
            fixture
                .package
                .installed_version()
                .unwrap()
                .to_string(),
            "7.0.2"
        );
    }

    #[test]
    fn a_failed_update_reports_an_error_if_the_old_binary_is_missing() {
        let fixture = Fixture::new();
        write_install(fixture.work_dir.path(), "7.0.2", true);
        let mut npm = fixture.npm(
            Ok("7.0.3"),
            InstallOutcome::FailAfterRemovingBinary,
        );
        fixture.assert_start_fails(&mut npm);
        assert!(
            fixture
                .package
                .installed_version()
                .is_none()
        );
    }

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
                let package = super::NativePackage::new(work_dir, os, arch).unwrap();
                let name = format!("@typescript/typescript-{platform}-{suffix}");
                assert_eq!(package.name, name);
                let directory = std::path::Path::new("node_modules").join(name);
                assert_eq!(
                    package.manifest,
                    directory.join("package.json")
                );
                assert_eq!(
                    package.binary,
                    directory.join("lib").join(executable)
                );
            }
        }
    }

    #[test]
    fn rejects_32_bit_x86() {
        let result = super::NativePackage::new(
            std::path::Path::new(""),
            z::Os::Windows,
            z::Architecture::X86,
        );
        assert!(result.is_err());
    }

    #[test]
    fn accepts_stable_typescript_7_or_newer() {
        for (version, supported) in [
            ("6.9.0", false),
            ("7.0.0-rc.1", false),
            ("7.0.0", true),
            ("8.0.0", true),
        ] {
            let version = semver::Version::parse(version).unwrap();
            assert_eq!(
                super::is_supported(&version),
                supported,
                "{version}"
            );
        }
    }
}
