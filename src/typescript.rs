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
    use zed_extension_api::LanguageServerInstallationStatus as Status;

    const PACKAGE_NAME: &str = "@typescript/typescript-win32-x64";
    const EXECUTABLE: &str = "tsc.exe";

    enum InstallOutcome {
        Complete,
        MissingBinary,
        WrongVersion,
        Fail,
        FailAfterRemovingBinary,
    }

    struct Registry {
        work_dir: std::path::PathBuf,
        latest: z::Result<&'static str>,
        outcome: InstallOutcome,
        installs: Vec<(String, String)>,
    }

    impl super::Npm for Registry {
        fn latest_version(&mut self, _name: &str) -> z::Result<String> {
            self.latest.clone().map(str::to_owned)
        }

        fn install(&mut self, name: &str, version: &str) -> z::Result<()> {
            self.installs
                .push((name.to_owned(), version.to_owned()));
            match self.outcome {
                InstallOutcome::Complete => {
                    write_manifest(&self.work_dir, name, version);
                    write_binary(&self.work_dir, name, EXECUTABLE);
                }
                InstallOutcome::MissingBinary => write_manifest(&self.work_dir, name, version),
                InstallOutcome::WrongVersion => {
                    write_manifest(&self.work_dir, name, "7.0.1");
                    write_binary(&self.work_dir, name, EXECUTABLE);
                }
                InstallOutcome::Fail => return Err("npm failed".into()),
                InstallOutcome::FailAfterRemovingBinary => {
                    std::fs::remove_file(binary_path(
                        &self.work_dir,
                        name,
                        EXECUTABLE,
                    ))
                    .unwrap();
                    return Err("npm failed".into());
                }
            }
            Ok(())
        }
    }

    struct StartFixture {
        sut: super::NativePackage,
        registry: Registry,
        statuses: Vec<Status>,
        directory: tempfile::TempDir,
    }

    impl StartFixture {
        fn new(latest: z::Result<&'static str>, outcome: InstallOutcome) -> Self {
            Self::for_platform(
                z::Os::Windows,
                z::Architecture::X8664,
                latest,
                outcome,
            )
        }

        fn for_platform(
            os: z::Os,
            arch: z::Architecture,
            latest: z::Result<&'static str>,
            outcome: InstallOutcome,
        ) -> Self {
            let directory = tempfile::tempdir().unwrap();
            let sut = super::NativePackage::new(directory.path(), os, arch).unwrap();
            Self {
                sut,
                registry: Registry {
                    work_dir: directory.path().to_owned(),
                    latest,
                    outcome,
                    installs: Vec::new(),
                },
                statuses: Vec::new(),
                directory,
            }
        }

        fn with_cached_package(self, name: &str, executable: &str, version: &str) -> Self {
            write_manifest(self.directory.path(), name, version);
            write_binary(self.directory.path(), name, executable);
            self
        }

        fn with_cached_version(self, version: &str) -> Self {
            self.with_cached_package(PACKAGE_NAME, EXECUTABLE, version)
        }

        fn start(&mut self) -> z::Result<std::path::PathBuf> {
            self.sut
                .ensure_executable(&mut self.registry, |status| {
                    self.statuses.push(status);
                })
        }

        fn assert_install_request(&self, version: &str) {
            assert_eq!(
                self.registry.installs,
                [(
                    PACKAGE_NAME.to_owned(),
                    version.to_owned()
                )],
            );
        }

        fn assert_no_install_requested(&self) {
            assert_eq!(
                self.registry.installs,
                Vec::<(String, String)>::new()
            );
        }

        fn assert_ready(&self, result: z::Result<std::path::PathBuf>) {
            assert_eq!(
                result.unwrap(),
                binary_path(
                    self.directory.path(),
                    PACKAGE_NAME,
                    EXECUTABLE
                ),
            );
            assert!(matches!(
                self.statuses.last(),
                Some(Status::None)
            ));
        }

        fn assert_start_failure(&self, result: z::Result<std::path::PathBuf>, reason: &str) {
            let error = result.unwrap_err();
            assert!(error.contains(reason), "{error}");
            assert!(
                matches!(self.statuses.last(), Some(Status::Failed(message)) if message == &error)
            );
        }
    }

    fn package_directory(work_dir: &std::path::Path, name: &str) -> std::path::PathBuf {
        work_dir.join("node_modules").join(name)
    }

    fn binary_path(work_dir: &std::path::Path, name: &str, executable: &str) -> std::path::PathBuf {
        package_directory(work_dir, name)
            .join("lib")
            .join(executable)
    }

    fn write_manifest(work_dir: &std::path::Path, name: &str, version: &str) {
        let directory = package_directory(work_dir, name);
        std::fs::create_dir_all(&directory).unwrap();
        let manifest = z::serde_json::json!({"name": name, "version": version});
        std::fs::write(
            directory.join("package.json"),
            manifest.to_string(),
        )
        .unwrap();
    }

    fn write_binary(work_dir: &std::path::Path, name: &str, executable: &str) {
        let binary = binary_path(work_dir, name, executable);
        std::fs::create_dir_all(binary.parent().unwrap()).unwrap();
        std::fs::write(binary, b"native executable").unwrap();
    }

    #[test]
    fn first_start_installs_the_published_version_and_returns_its_executable() {
        let mut fixture = StartFixture::new(Ok("7.0.2"), InstallOutcome::Complete);

        let result = fixture.start();

        fixture.assert_ready(result);
        fixture.assert_install_request("7.0.2");
        assert!(matches!(
            fixture.statuses.as_slice(),
            [
                Status::CheckingForUpdate,
                Status::Downloading,
                Status::None
            ]
        ));
    }

    #[test]
    fn first_start_rejects_a_prerelease() {
        let mut fixture = StartFixture::new(
            Ok("7.0.0-rc.1"),
            InstallOutcome::Complete,
        );

        let result = fixture.start();

        fixture.assert_start_failure(result, "unsupported");
        fixture.assert_no_install_requested();
    }

    #[test]
    fn first_start_rejects_typescript_older_than_7() {
        let mut fixture = StartFixture::new(Ok("6.9.0"), InstallOutcome::Complete);

        let result = fixture.start();

        fixture.assert_start_failure(result, "unsupported");
        fixture.assert_no_install_requested();
    }

    #[test]
    fn an_up_to_date_install_can_start_without_a_download() {
        let mut fixture =
            StartFixture::new(Ok("7.0.2"), InstallOutcome::Fail).with_cached_version("7.0.2");

        let result = fixture.start();

        fixture.assert_ready(result);
        fixture.assert_no_install_requested();
    }

    #[test]
    fn an_older_registry_release_leaves_the_current_install_available() {
        let mut fixture =
            StartFixture::new(Ok("7.0.1"), InstallOutcome::Fail).with_cached_version("7.0.2");

        let result = fixture.start();

        fixture.assert_ready(result);
        fixture.assert_no_install_requested();
    }

    #[test]
    fn a_build_metadata_change_does_not_trigger_a_download() {
        let mut fixture =
            StartFixture::new(Ok("7.0.2+build"), InstallOutcome::Fail).with_cached_version("7.0.2");

        let result = fixture.start();

        fixture.assert_ready(result);
        fixture.assert_no_install_requested();
    }

    #[test]
    fn a_cached_install_can_start_when_the_registry_is_offline() {
        let mut fixture = StartFixture::new(
            Err("offline".into()),
            InstallOutcome::Fail,
        )
        .with_cached_version("7.0.2");

        let result = fixture.start();

        fixture.assert_ready(result);
        fixture.assert_no_install_requested();
    }

    #[test]
    fn a_cached_prerelease_cannot_start_when_the_registry_is_offline() {
        let mut fixture = StartFixture::new(
            Err("offline".into()),
            InstallOutcome::Fail,
        )
        .with_cached_version("7.0.0-rc.1");

        let result = fixture.start();

        fixture.assert_start_failure(result, "offline");
        fixture.assert_no_install_requested();
    }

    #[test]
    fn a_download_without_an_executable_cannot_start() {
        let mut fixture = StartFixture::new(
            Ok("7.0.2"),
            InstallOutcome::MissingBinary,
        );

        let result = fixture.start();

        fixture.assert_start_failure(result, "missing or incomplete");
    }

    #[test]
    fn a_cached_install_can_upgrade_to_a_newer_stable_major_release() {
        let mut fixture =
            StartFixture::new(Ok("8.0.0"), InstallOutcome::Complete).with_cached_version("7.0.2");

        let result = fixture.start();

        fixture.assert_ready(result);
        fixture.assert_install_request("8.0.0");
    }

    #[test]
    fn a_download_of_the_wrong_version_cannot_start() {
        let mut fixture = StartFixture::new(
            Ok("7.0.2"),
            InstallOutcome::WrongVersion,
        );

        let result = fixture.start();

        fixture.assert_start_failure(result, "missing or incomplete");
    }

    #[test]
    fn an_intact_cached_install_can_start_after_a_failed_update() {
        let mut fixture =
            StartFixture::new(Ok("7.0.3"), InstallOutcome::Fail).with_cached_version("7.0.2");

        let result = fixture.start();

        fixture.assert_ready(result);
        fixture.assert_install_request("7.0.3");
    }

    #[test]
    fn a_failed_update_that_removes_the_old_binary_prevents_startup() {
        let mut fixture = StartFixture::new(
            Ok("7.0.3"),
            InstallOutcome::FailAfterRemovingBinary,
        )
        .with_cached_version("7.0.2");

        let result = fixture.start();

        fixture.assert_start_failure(result, "npm failed");
    }

    macro_rules! cached_platform_starts {
        ($test:ident, $os:ident, $arch:ident, $package:literal, $binary:literal) => {
            #[test]
            fn $test() {
                let mut fixture = StartFixture::for_platform(
                    z::Os::$os,
                    z::Architecture::$arch,
                    Err("offline".into()),
                    InstallOutcome::Fail,
                )
                .with_cached_package($package, $binary, "7.0.2");

                let result = fixture.start();

                assert_eq!(
                    result.unwrap(),
                    binary_path(
                        fixture.directory.path(),
                        $package,
                        $binary
                    ),
                );
            }
        };
    }

    cached_platform_starts!(
        linux_arm64_starts_its_native_binary,
        Linux,
        Aarch64,
        "@typescript/typescript-linux-arm64",
        "tsc"
    );
    cached_platform_starts!(
        linux_x64_starts_its_native_binary,
        Linux,
        X8664,
        "@typescript/typescript-linux-x64",
        "tsc"
    );
    cached_platform_starts!(
        mac_arm64_starts_its_native_binary,
        Mac,
        Aarch64,
        "@typescript/typescript-darwin-arm64",
        "tsc"
    );
    cached_platform_starts!(
        mac_x64_starts_its_native_binary,
        Mac,
        X8664,
        "@typescript/typescript-darwin-x64",
        "tsc"
    );
    cached_platform_starts!(
        windows_arm64_starts_its_native_binary,
        Windows,
        Aarch64,
        "@typescript/typescript-win32-arm64",
        "tsc.exe"
    );

    #[test]
    fn a_32_bit_platform_cannot_start_typescript_7() {
        let result = super::NativePackage::new(
            std::path::Path::new(""),
            z::Os::Windows,
            z::Architecture::X86,
        );

        assert_eq!(
            result.err().as_deref(),
            Some("TypeScript 7 requires a 64-bit platform"),
        );
    }
}
