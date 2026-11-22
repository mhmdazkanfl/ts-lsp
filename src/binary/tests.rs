use std::{
    cell::Cell,
    sync::atomic::{AtomicUsize, Ordering},
};

use zed_extension_api::serde_json::json;

use super::*;

struct ManagedInstall {
    modules: PathBuf,
    platform: PlatformPackage,
}

impl ManagedInstall {
    fn new() -> Self {
        static NEXT_DIRECTORY: AtomicUsize = AtomicUsize::new(0);
        let root = std::env::temp_dir();
        let modules = loop {
            let id = NEXT_DIRECTORY.fetch_add(1, Ordering::Relaxed);
            let path = root.join(format!("ts-lsp-test-{}-{id}", std::process::id()));
            match fs::create_dir(&path) {
                Ok(()) => break path,
                Err(error) if error.kind() == ErrorKind::AlreadyExists => continue,
                Err(error) => panic!("Could not create {}: {error}", path.display()),
            }
        };
        Self {
            modules,
            platform: PlatformPackage {
                name: "@typescript/typescript-test-platform".into(),
                binary_name: "tsc",
            },
        }
    }

    fn platform_dir(&self) -> PathBuf {
        self.modules.join(&self.platform.name)
    }

    fn write_version(&self, version: &str) -> PathBuf {
        for name in [PACKAGE_NAME, &self.platform.name] {
            let directory = self.modules.join(name);
            fs::create_dir_all(&directory).unwrap();
            let manifest = json!({ "name": name, "version": version });
            fs::write(directory.join("package.json"), manifest.to_string()).unwrap();
        }
        let lib = self.platform_dir().join("lib");
        fs::create_dir_all(&lib).unwrap();
        let binary = lib.join(self.platform.binary_name);
        fs::write(&binary, []).unwrap();
        binary
    }

    fn resolve(
        &self,
        resolver: &mut BinaryResolver,
        selector: Option<&str>,
        npm: &impl Npm,
    ) -> Result<PathBuf> {
        let pin = selector.and_then(|version| version.parse().ok());
        resolver.resolve_managed(&self.platform, &self.modules, selector, pin.as_ref(), npm)
    }
}

impl Drop for ManagedInstall {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.modules);
    }
}

struct TestNpm<F> {
    latest: Result<&'static str, &'static str>,
    latest_checks: Cell<usize>,
    install: F,
}

impl<F: Fn(&str) -> Result<()>> TestNpm<F> {
    fn new(latest: Result<&'static str, &'static str>, install: F) -> Self {
        Self {
            latest,
            latest_checks: Cell::new(0),
            install,
        }
    }
}

impl<F: Fn(&str) -> Result<()>> Npm for TestNpm<F> {
    fn latest_version(&self) -> Result<String> {
        self.latest_checks.update(|checks| checks + 1);
        self.latest.map(str::to_owned).map_err(str::to_owned)
    }

    fn install(&self, target: &str) -> Result<()> {
        (self.install)(target)
    }

    fn set_status(&self, _: Status) {}
}

#[test]
fn reuses_exact_pin_and_caches_an_unpinned_latest_check() {
    let managed = ManagedInstall::new();
    let binary = managed.write_version("7.0.2");
    let npm = TestNpm::new(Ok("7.0.2"), |_| panic!("unexpected install"));
    let mut resolver = BinaryResolver::default();

    assert_eq!(
        managed.resolve(&mut resolver, Some("7.0.2"), &npm),
        Ok(binary.clone())
    );
    assert_eq!(npm.latest_checks.get(), 0);

    assert_eq!(
        managed.resolve(&mut resolver, None, &npm),
        Ok(binary.clone())
    );
    assert_eq!(managed.resolve(&mut resolver, None, &npm), Ok(binary));
    assert_eq!(npm.latest_checks.get(), 1);
}

#[test]
fn installs_a_different_exact_pin_and_rejects_an_incorrect_npm_result() {
    let managed = ManagedInstall::new();
    let binary = managed.write_version("7.0.2");
    let npm = TestNpm::new(Err("unexpected latest check"), |target| {
        assert_eq!(target, "7.0.3");
        assert!(binary.is_file());
        managed.write_version("7.0.3");
        Ok(())
    });
    let mut resolver = BinaryResolver::default();

    assert_eq!(
        managed.resolve(&mut resolver, Some("7.0.3"), &npm).unwrap(),
        binary
    );
    assert_eq!(npm.latest_checks.get(), 0);

    let npm = TestNpm::new(Err("unexpected latest check"), |target| {
        assert_eq!(target, "7.0.4");
        Ok(())
    });
    let error = managed
        .resolve(&mut resolver, Some("7.0.4"), &npm)
        .unwrap_err();
    assert_eq!(error, "Requested TypeScript 7.0.4, but npm installed 7.0.3");
}

#[test]
fn forwards_tags_and_ranges_to_npm() {
    let managed = ManagedInstall::new();
    let binary = managed.write_version("7.0.2");
    let mut resolver = BinaryResolver::default();

    for selector in ["next", "^7.0.0"] {
        let installs = Cell::new(0);
        let npm = TestNpm::new(Err("unexpected latest check"), |target| {
            assert_eq!(target, selector);
            installs.set(installs.get() + 1);
            Ok(())
        });
        assert_eq!(
            managed.resolve(&mut resolver, Some(selector), &npm),
            Ok(binary.clone())
        );
        assert_eq!(installs.get(), 1);
        assert_eq!(npm.latest_checks.get(), 0);
    }
}

#[test]
fn reuses_a_valid_install_when_the_latest_lookup_fails() {
    let managed = ManagedInstall::new();
    let binary = managed.write_version("7.0.2");
    let npm = TestNpm::new(Err("registry unavailable"), |_| {
        panic!("unexpected install")
    });

    assert_eq!(
        managed.resolve(&mut BinaryResolver::default(), None, &npm),
        Ok(binary)
    );
    assert_eq!(npm.latest_checks.get(), 1);
}

#[test]
fn reuses_a_valid_install_after_a_failed_update_only_without_a_selector() {
    let managed = ManagedInstall::new();
    let binary = managed.write_version("7.0.2");
    let npm = TestNpm::new(Ok("7.0.3"), |target| {
        assert_eq!(target, "7.0.3");
        Err("download failed".into())
    });
    let mut resolver = BinaryResolver::default();

    assert_eq!(managed.resolve(&mut resolver, None, &npm), Ok(binary));
    let error = managed
        .resolve(&mut resolver, Some("7.0.3"), &npm)
        .unwrap_err();
    assert_eq!(error, "Could not install TypeScript 7.0.3: download failed");
}

#[test]
fn reports_failed_update_and_broken_fallback_after_rechecking_disk() {
    let managed = ManagedInstall::new();
    let binary = managed.write_version("7.0.2");
    let npm = TestNpm::new(Ok("7.0.3"), |_| {
        fs::remove_file(&binary).unwrap();
        Err("download failed".into())
    });

    let error = managed
        .resolve(&mut BinaryResolver::default(), None, &npm)
        .unwrap_err();
    assert!(error.contains("download failed"), "{error}");
    assert!(
        error.contains("Native TypeScript executable is missing"),
        "{error}"
    );
    assert!(error.contains(&binary.display().to_string()), "{error}");
}

#[test]
fn repairs_a_missing_or_invalid_manifest_or_executable() {
    let managed = ManagedInstall::new();
    let directory = managed.platform_dir();
    let manifest = directory.join("package.json");
    let repair = || managed.platform.remove_incomplete_package(&managed.modules);

    let binary = managed.write_version("7.0.2");
    repair().unwrap();
    assert!(binary.is_file());

    fs::remove_file(binary).unwrap();
    repair().unwrap();
    assert!(!directory.exists());

    managed.write_version("7.0.2");
    fs::remove_file(&manifest).unwrap();
    repair().unwrap();
    assert!(!directory.exists());

    managed.write_version("7.0.2");
    fs::write(manifest, "invalid JSON").unwrap();
    repair().unwrap();
    assert!(!directory.exists());
    repair().unwrap();
}

#[test]
fn surfaces_manifest_io_errors_and_leaves_the_package_in_place() {
    let managed = ManagedInstall::new();
    let directory = managed.platform_dir();
    let manifest = directory.join("package.json");
    fs::create_dir_all(&manifest).unwrap();
    let read_error = fs::read(&manifest).unwrap_err();
    let npm = TestNpm::new(Ok("7.0.2"), |_| panic!("unexpected install"));

    let error = managed
        .resolve(&mut BinaryResolver::default(), None, &npm)
        .unwrap_err();
    assert_eq!(
        error,
        format!("Could not read {}: {read_error}", manifest.display())
    );
    assert!(manifest.is_dir());
}
