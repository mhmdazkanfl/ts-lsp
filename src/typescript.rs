mod package;
mod release;

use zed_extension_api as z;

struct ZedNpm;

impl package::Npm for ZedNpm {
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
    package::prepare_server(
        std::path::Path::new(""),
        z::current_platform(),
        &mut ZedNpm,
        |status| z::set_language_server_installation_status(language_server_id, &status),
    )
}

#[cfg(test)]
mod tests {
    use zed_extension_api as z;

    const PLATFORM: (z::Os, z::Architecture) = (z::Os::Linux, z::Architecture::X8664);
    const PACKAGE_NAME: &str = "@typescript/typescript-linux-x64";
    const SERVER_PATH: &str = "node_modules/@typescript/typescript-linux-x64/lib/tsc";
    const MANIFEST_PATH: &str = "node_modules/@typescript/typescript-linux-x64/package.json";

    struct Registry<I> {
        latest: z::Result<&'static str>,
        apply_install: I,
        installs: Vec<(String, String)>,
    }

    fn registry<I>(latest: z::Result<&'static str>, apply_install: I) -> Registry<I>
    where
        I: FnMut(&str) -> z::Result<()>,
    {
        Registry {
            latest,
            apply_install,
            installs: Vec::new(),
        }
    }

    fn lookup_only(latest: z::Result<&'static str>) -> impl super::package::Npm {
        registry(latest, |_| {
            Err("unexpected install".into())
        })
    }

    impl<I: FnMut(&str) -> z::Result<()>> super::package::Npm for Registry<I> {
        fn latest_version(&mut self, name: &str) -> z::Result<String> {
            if name != PACKAGE_NAME {
                return Err(format!("Package {name} was not found"));
            }
            self.latest.clone().map(str::to_owned)
        }

        fn install(&mut self, name: &str, version: &str) -> z::Result<()> {
            self.installs
                .push((name.into(), version.into()));
            (self.apply_install)(version)
        }
    }

    impl<I> Registry<I> {
        fn assert_one_install(&self, version: &str) {
            assert_eq!(
                self.installs,
                [(PACKAGE_NAME.into(), version.into())]
            );
        }

        fn assert_no_install(&self) {
            assert_eq!(
                self.installs,
                Vec::<(String, String)>::new()
            );
        }
    }

    struct WorkDirectory(tempfile::TempDir);

    impl WorkDirectory {
        fn new() -> Self {
            Self(tempfile::tempdir().unwrap())
        }

        fn binary(&self) -> std::path::PathBuf {
            self.0.path().join(SERVER_PATH)
        }

        fn write(&self, location: &str, contents: &[u8]) {
            let path = self.0.path().join(location);
            std::fs::create_dir_all(path.parent().unwrap()).unwrap();
            std::fs::write(path, contents).unwrap();
        }

        fn write_manifest(&self, version: &str) {
            let manifest = z::serde_json::json!({ "name": PACKAGE_NAME, "version": version });
            self.write(
                MANIFEST_PATH,
                manifest.to_string().as_bytes(),
            );
        }

        fn install(&self, version: &str) {
            self.write(SERVER_PATH, b"test server");
            self.write_manifest(version);
        }

        fn with_release(version: &str) -> Self {
            let directory = Self::new();
            directory.install(version);
            directory
        }

        fn start(&self, npm: &mut impl super::package::Npm) -> Startup {
            let mut statuses = Vec::new();
            let file = super::package::prepare_server(self.0.path(), PLATFORM, npm, |status| {
                statuses.push(status);
            });
            Startup {
                file,
                statuses,
            }
        }
    }

    struct Startup {
        file: z::Result<std::path::PathBuf>,
        statuses: Vec<z::LanguageServerInstallationStatus>,
    }

    impl Startup {
        fn used_cached_server(self, binary: &std::path::Path) {
            assert_eq!(self.file.unwrap(), binary);
            assert!(
                matches!(
                    self.statuses.as_slice(),
                    [
                        z::LanguageServerInstallationStatus::CheckingForUpdate,
                        z::LanguageServerInstallationStatus::None
                    ]
                ),
                "{:?}",
                self.statuses
            );
        }

        fn used_server_after_download(self, binary: &std::path::Path) {
            assert_eq!(self.file.unwrap(), binary);
            assert!(
                matches!(
                    self.statuses.as_slice(),
                    [
                        z::LanguageServerInstallationStatus::CheckingForUpdate,
                        z::LanguageServerInstallationStatus::Downloading,
                        z::LanguageServerInstallationStatus::None
                    ]
                ),
                "{:?}",
                self.statuses
            );
        }

        fn failed_during_lookup(self, reason: &str) {
            let error = self.file.unwrap_err();
            assert!(error.contains(reason), "{error}");
            assert!(
                matches!(
                    self.statuses.as_slice(),
                    [z::LanguageServerInstallationStatus::CheckingForUpdate, z::LanguageServerInstallationStatus::Failed(message)] if message == &error
                ),
                "{:?}",
                self.statuses
            );
        }

        fn failed_during_download(self, reason: &str) {
            let error = self.file.unwrap_err();
            assert!(error.contains(reason), "{error}");
            assert!(
                matches!(
                    self.statuses.as_slice(),
                    [z::LanguageServerInstallationStatus::CheckingForUpdate, z::LanguageServerInstallationStatus::Downloading, z::LanguageServerInstallationStatus::Failed(message)]
                        if message == &error
                ),
                "{:?}",
                self.statuses
            );
        }
    }

    #[test]
    fn a_32_bit_system_gets_an_error_before_installation_starts() {
        let directory = WorkDirectory::new();
        let mut npm = registry(Ok("7.0.2"), |_| {
            Err("unexpected install".into())
        });
        let mut statuses = Vec::new();

        let result = super::package::prepare_server(
            directory.0.path(),
            (z::Os::Windows, z::Architecture::X86),
            &mut npm,
            |status| statuses.push(status),
        );

        assert_eq!(
            result.unwrap_err(),
            "TypeScript 7 requires a 64-bit platform"
        );
        assert!(statuses.is_empty());
        npm.assert_no_install();
    }

    #[test]
    fn first_start_downloads_the_exact_published_release() {
        let directory = WorkDirectory::new();
        let mut npm = registry(Ok("7.0.2+build.4"), |version| {
            directory.install(version);
            Ok(())
        });

        let startup = directory.start(&mut npm);

        startup.used_server_after_download(&directory.binary());
        npm.assert_one_install("7.0.2+build.4");
    }

    #[test]
    fn a_current_server_is_used_without_downloading_again() {
        let directory = WorkDirectory::with_release("7.0.2");
        let mut npm = registry(Ok("7.0.2"), |_| {
            Err("unexpected install".into())
        });

        let startup = directory.start(&mut npm);

        startup.used_cached_server(&directory.binary());
        npm.assert_no_install();
    }

    #[test]
    fn a_new_stable_major_version_can_replace_the_cached_server() {
        let directory = WorkDirectory::with_release("7.0.2");
        let mut npm = registry(Ok("8.0.0"), |version| {
            directory.install(version);
            Ok(())
        });

        let startup = directory.start(&mut npm);

        startup.used_server_after_download(&directory.binary());
        npm.assert_one_install("8.0.0");
    }

    #[test]
    fn a_stable_download_can_replace_an_unsupported_cached_server() {
        let directory = WorkDirectory::with_release("7.1.0-rc.1");
        let mut npm = registry(Ok("7.0.2"), |version| {
            directory.install(version);
            Ok(())
        });

        let startup = directory.start(&mut npm);

        startup.used_server_after_download(&directory.binary());
        npm.assert_one_install("7.0.2");
    }

    macro_rules! lookup_failures {
        ($($test:ident: $latest:expr => $reason:literal,)*) => {
            $(
                #[test]
                fn $test() {
                    let directory = WorkDirectory::new();
                    let mut npm = registry($latest, |_| Err("unexpected install".into()));

                    let startup = directory.start(&mut npm);

                    startup.failed_during_lookup($reason);
                    npm.assert_no_install();
                }
            )*
        };
    }

    lookup_failures! {
        first_start_fails_if_npm_cannot_be_reached: Err("offline".into()) => "offline",
        first_start_fails_if_npm_returns_an_invalid_version: Ok("latest") => "Invalid version latest",
        first_start_fails_if_npm_only_offers_an_old_release: Ok("6.9.0") => "is unsupported",
        first_start_fails_if_npm_only_offers_a_prerelease: Ok("7.1.0-rc.1") => "is unsupported",
    }

    macro_rules! lookup_recoveries {
        ($($test:ident: $latest:expr,)*) => {
            $(
                #[test]
                fn $test() {
                    let directory = WorkDirectory::with_release("7.0.2");
                    let mut npm = registry($latest, |_| Err("unexpected install".into()));

                    let startup = directory.start(&mut npm);

                    startup.used_cached_server(&directory.binary());
                    npm.assert_no_install();
                }
            )*
        };
    }

    lookup_recoveries! {
        the_saved_server_is_used_when_npm_is_offline: Err("offline".into()),
        the_saved_server_is_used_when_npm_returns_an_invalid_version: Ok("latest"),
        the_saved_server_is_used_when_npm_offers_an_unsupported_release: Ok("7.1.0-rc.1"),
    }

    #[test]
    fn a_package_without_the_server_file_cannot_be_used_offline() {
        let directory = WorkDirectory::new();
        directory.write_manifest("7.0.2");
        let mut npm = lookup_only(Err("offline".into()));

        let startup = directory.start(&mut npm);

        startup.failed_during_lookup("offline");
    }

    #[test]
    fn an_empty_server_file_cannot_be_used_offline() {
        let directory = WorkDirectory::new();
        directory.write_manifest("7.0.2");
        directory.write(SERVER_PATH, b"");
        let mut npm = lookup_only(Err("offline".into()));

        let startup = directory.start(&mut npm);

        startup.failed_during_lookup("offline");
    }

    #[test]
    fn a_directory_at_the_server_path_cannot_be_used_offline() {
        let directory = WorkDirectory::new();
        directory.write_manifest("7.0.2");
        std::fs::create_dir_all(directory.binary()).unwrap();
        let mut npm = lookup_only(Err("offline".into()));

        let startup = directory.start(&mut npm);

        startup.failed_during_lookup("offline");
    }

    #[test]
    fn a_server_without_a_package_manifest_cannot_be_used_offline() {
        let directory = WorkDirectory::new();
        directory.write(SERVER_PATH, b"test server");
        let mut npm = lookup_only(Err("offline".into()));

        let startup = directory.start(&mut npm);

        startup.failed_during_lookup("offline");
    }

    macro_rules! unusable_manifests {
        ($($test:ident: $manifest:expr,)*) => {
            $(
                #[test]
                fn $test() {
                    let directory = WorkDirectory::new();
                    directory.write(SERVER_PATH, b"test server");
                    directory.write(MANIFEST_PATH, $manifest);
                    let mut npm = lookup_only(Err("offline".into()));

                    let startup = directory.start(&mut npm);

                    startup.failed_during_lookup("offline");
                }
            )*
        };
    }

    unusable_manifests! {
        invalid_utf8_prevents_offline_startup: b"\xff",
        invalid_json_prevents_offline_startup: b"{",
        a_missing_package_name_prevents_offline_startup: br#"{"version":"7.0.2"}"#,
        a_non_string_package_name_prevents_offline_startup: br#"{"name":7,"version":"7.0.2"}"#,
        the_wrong_package_name_prevents_offline_startup: br#"{"name":"@typescript/typescript-linux-arm64","version":"7.0.2"}"#,
        a_missing_version_prevents_offline_startup: br#"{"name":"@typescript/typescript-linux-x64"}"#,
        a_non_string_version_prevents_offline_startup: br#"{"name":"@typescript/typescript-linux-x64","version":7}"#,
        a_malformed_version_prevents_offline_startup: br#"{"name":"@typescript/typescript-linux-x64","version":"latest"}"#,
        a_cached_old_release_prevents_offline_startup: br#"{"name":"@typescript/typescript-linux-x64","version":"6.9.0"}"#,
        a_cached_prerelease_prevents_offline_startup: br#"{"name":"@typescript/typescript-linux-x64","version":"7.1.0-rc.1"}"#,
    }

    #[test]
    fn a_first_download_error_is_reported_to_the_editor() {
        let directory = WorkDirectory::new();
        let mut npm = registry(
            Ok("7.0.2"),
            |_| Err("npm failed".into()),
        );

        let startup = directory.start(&mut npm);

        startup.failed_during_download(
            "Failed to install @typescript/typescript-linux-x64@7.0.2: npm failed",
        );
        npm.assert_one_install("7.0.2");
    }

    #[test]
    fn npm_success_without_a_server_file_is_an_installation_error() {
        let directory = WorkDirectory::new();
        let mut npm = registry(Ok("7.0.2"), |version| {
            directory.write_manifest(version);
            Ok(())
        });

        let startup = directory.start(&mut npm);

        startup.failed_during_download("missing or incomplete");
        npm.assert_one_install("7.0.2");
    }

    #[test]
    fn a_download_of_the_wrong_release_is_an_installation_error() {
        let directory = WorkDirectory::new();
        let mut npm = registry(Ok("7.0.2"), |_| {
            directory.install("7.0.1");
            Ok(())
        });

        let startup = directory.start(&mut npm);

        startup.failed_during_download("missing or incomplete");
        npm.assert_one_install("7.0.2");
    }

    #[test]
    fn a_download_with_the_wrong_build_label_is_an_installation_error() {
        let directory = WorkDirectory::new();
        let mut npm = registry(Ok("7.0.2+requested"), |_| {
            directory.install("7.0.2+other");
            Ok(())
        });

        let startup = directory.start(&mut npm);

        startup.failed_during_download("missing or incomplete");
        npm.assert_one_install("7.0.2+requested");
    }

    #[test]
    fn a_failed_update_can_use_the_intact_old_server() {
        let directory = WorkDirectory::with_release("7.0.2");
        let mut npm = registry(
            Ok("7.0.3"),
            |_| Err("npm failed".into()),
        );

        let startup = directory.start(&mut npm);

        startup.used_server_after_download(&directory.binary());
        npm.assert_one_install("7.0.3");
    }

    #[test]
    fn a_claimed_update_that_leaves_the_old_server_can_still_start_it() {
        let directory = WorkDirectory::with_release("7.0.2");
        let mut npm = registry(Ok("7.0.3"), |_| Ok(()));

        let startup = directory.start(&mut npm);

        startup.used_server_after_download(&directory.binary());
        npm.assert_one_install("7.0.3");
    }

    #[test]
    fn a_failed_update_that_removes_the_old_server_prevents_startup() {
        let directory = WorkDirectory::with_release("7.0.2");
        let mut npm = registry(Ok("7.0.3"), |_| {
            std::fs::remove_file(directory.binary()).unwrap();
            Err("npm failed".into())
        });

        let startup = directory.start(&mut npm);

        startup.failed_during_download("npm failed");
        npm.assert_one_install("7.0.3");
    }

    #[test]
    fn a_failed_update_that_changes_the_build_label_cannot_use_the_old_server() {
        let directory = WorkDirectory::with_release("7.0.2+original");
        let mut npm = registry(Ok("7.0.3"), |_| {
            directory.write_manifest("7.0.2+changed");
            Err("npm failed".into())
        });

        let startup = directory.start(&mut npm);

        startup.failed_during_download("npm failed");
        npm.assert_one_install("7.0.3");
    }

    #[test]
    fn a_failed_update_does_not_start_the_new_server_even_if_its_files_exist() {
        let directory = WorkDirectory::with_release("7.0.2");
        let mut npm = registry(Ok("7.0.3"), |version| {
            directory.install(version);
            Err("npm failed".into())
        });

        let startup = directory.start(&mut npm);

        startup.failed_during_download("npm failed");
        npm.assert_one_install("7.0.3");
    }
}
