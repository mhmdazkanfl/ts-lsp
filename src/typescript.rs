mod package;
mod update;

use zed_extension_api as z;

pub fn ensure_installed(
    os: z::Os,
    arch: z::Architecture,
    report: impl FnMut(z::LanguageServerInstallationStatus),
) -> z::Result<std::path::PathBuf> {
    let package = package::PlatformPackage::new(std::path::Path::new(""), os, arch)?;
    ensure_package(&package, &package::ZedNpm, report)?;
    Ok(package.into_binary())
}

fn ensure_package(
    package: &package::PlatformPackage,
    npm: &impl package::Npm,
    mut report: impl FnMut(z::LanguageServerInstallationStatus),
) -> z::Result<()> {
    let installed = package
        .installed_version()
        .filter(update::is_supported);
    report(z::LanguageServerInstallationStatus::CheckingForUpdate);

    let attempt = (|| {
        let available = package.latest_version(npm)?;
        if !update::is_supported(&available) {
            return Err(format!(
                "{}@{available} is unsupported. Expected a stable TypeScript version 7 or newer",
                package.name()
            ));
        }
        if update::should_install(installed.as_ref(), &available) {
            report(z::LanguageServerInstallationStatus::Downloading);
            package.install(npm, &available)?;
        }
        Ok(())
    })();

    if let Err(err) = attempt {
        if installed.is_none() || package.installed_version() != installed {
            report(z::LanguageServerInstallationStatus::Failed(err.clone()));
            return Err(err);
        }
        eprintln!("Using the installed TypeScript language server after an update failed: {err}");
    }

    report(z::LanguageServerInstallationStatus::None);
    Ok(())
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
        checks: std::cell::Cell<usize>,
        versions_requested: std::cell::RefCell<Vec<String>>,
    }

    impl super::package::Npm for FakeNpm<'_> {
        fn latest_version(&self, name: &str) -> z::Result<String> {
            assert_eq!(name, PACKAGE_NAME);
            self.checks.set(self.checks.get() + 1);
            self.latest.clone().map(str::to_owned)
        }

        fn install(&self, name: &str, version: &str) -> z::Result<()> {
            assert_eq!(name, PACKAGE_NAME);
            self.versions_requested
                .borrow_mut()
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
        package: super::package::PlatformPackage,
    }

    impl Fixture {
        fn new() -> Self {
            let work_dir = tempfile::tempdir().unwrap();
            let package = super::package::PlatformPackage::new(
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
                checks: std::cell::Cell::default(),
                versions_requested: std::cell::RefCell::default(),
            }
        }

        fn start(&self, npm: &FakeNpm<'_>) -> z::Result<()> {
            super::ensure_package(&self.package, npm, |_| {})
        }

        fn assert_start_fails(&self, npm: &FakeNpm<'_>) {
            let mut last_status = None;
            let result = super::ensure_package(&self.package, npm, |status| {
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
        let npm = fixture.npm(Ok("7.0.2"), InstallOutcome::Complete);
        fixture.start(&npm).unwrap();
        assert_eq!(
            *npm.versions_requested.borrow(),
            ["7.0.2"]
        );
    }

    #[test]
    fn cold_start_rejects_an_unsupported_release_without_downloading_it() {
        let fixture = Fixture::new();
        let npm = fixture.npm(
            Ok("7.0.0-rc.1"),
            InstallOutcome::Complete,
        );

        fixture.assert_start_fails(&npm);

        assert!(
            npm.versions_requested
                .borrow()
                .is_empty()
        );
    }

    #[test]
    fn each_start_checks_npm_and_reuses_the_current_version() {
        let fixture = Fixture::new();
        write_install(fixture.work_dir.path(), "7.0.2", true);
        let npm = fixture.npm(Ok("7.0.2"), InstallOutcome::Fail);
        fixture.start(&npm).unwrap();
        fixture.start(&npm).unwrap();
        assert_eq!(npm.checks.get(), 2);
        assert!(
            npm.versions_requested
                .borrow()
                .is_empty()
        );
    }

    #[test]
    fn a_failed_version_lookup_reuses_the_installed_binary() {
        let fixture = Fixture::new();
        write_install(fixture.work_dir.path(), "7.0.2", true);
        let npm = fixture.npm(
            Err("offline".into()),
            InstallOutcome::Fail,
        );
        fixture.start(&npm).unwrap();
        assert_eq!(npm.checks.get(), 1);
        assert!(
            npm.versions_requested
                .borrow()
                .is_empty()
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
        let npm = fixture.npm(
            Err("offline".into()),
            InstallOutcome::Fail,
        );

        fixture.assert_start_fails(&npm);
    }

    #[test]
    fn npm_success_requires_an_executable() {
        let fixture = Fixture::new();
        let npm = fixture.npm(
            Ok("7.0.2"),
            InstallOutcome::MissingBinary,
        );
        fixture.assert_start_fails(&npm);
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
        let npm = fixture.npm(Ok("7.0.3"), InstallOutcome::Complete);
        fixture.start(&npm).unwrap();
        assert_eq!(
            fixture.package.installed_version(),
            Some(semver::Version::parse("7.0.3").unwrap())
        );
    }

    #[test]
    fn npm_success_requires_the_expected_version() {
        let fixture = Fixture::new();
        let npm = fixture.npm(
            Ok("7.0.2"),
            InstallOutcome::WrongVersion,
        );
        fixture.assert_start_fails(&npm);
        assert_eq!(
            fixture.package.installed_version(),
            Some(semver::Version::parse("7.0.1").unwrap())
        );
    }

    #[test]
    fn a_failed_update_reuses_the_previous_install_if_it_is_intact() {
        let fixture = Fixture::new();
        write_install(fixture.work_dir.path(), "7.0.2", true);
        let npm = fixture.npm(Ok("7.0.3"), InstallOutcome::Fail);
        fixture.start(&npm).unwrap();
        assert_eq!(
            *npm.versions_requested.borrow(),
            ["7.0.3"]
        );
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
        let npm = fixture.npm(
            Ok("7.0.3"),
            InstallOutcome::FailAfterRemovingBinary,
        );
        fixture.assert_start_fails(&npm);
        assert!(
            fixture
                .package
                .installed_version()
                .is_none()
        );
    }
}
