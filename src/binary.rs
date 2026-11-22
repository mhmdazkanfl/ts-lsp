use std::{
    fs,
    io::{self, ErrorKind},
    path::{Path, PathBuf},
};

use semver::Version;
use zed_extension_api::{
    self as zed, LanguageServerId, LanguageServerInstallationStatus as Status, Result,
};
use zed_extension_api::{Worktree, serde_json::Value, settings::CommandSettings};

#[cfg(test)]
mod tests;

const PACKAGE_NAME: &str = "typescript";
const DEFAULT_ARGUMENTS: [&str; 2] = ["--lsp", "--stdio"];

#[derive(Default)]
pub(crate) struct BinaryResolver {
    checked_latest: Option<Version>,
}

#[derive(Debug, PartialEq, Eq)]
struct ManagedBinary {
    path: PathBuf,
    version: Version,
}

struct PlatformPackage {
    name: String,
    binary_name: &'static str,
}

impl PlatformPackage {
    fn current() -> Result<Self> {
        let (os, architecture) = zed::current_platform();
        let (os, binary_name) = match os {
            zed::Os::Mac => ("darwin", "tsc"),
            zed::Os::Linux => ("linux", "tsc"),
            zed::Os::Windows => ("win32", "tsc.exe"),
        };
        let architecture = match architecture {
            zed::Architecture::Aarch64 => "arm64",
            zed::Architecture::X8664 => "x64",
            zed::Architecture::X86 => {
                return Err("TypeScript Native requires a 64-bit system".into());
            }
        };
        Ok(Self {
            name: format!("@typescript/typescript-{os}-{architecture}"),
            binary_name,
        })
    }

    fn project_binary(&self, worktree: &Worktree, pin: Option<&Version>) -> Result<Option<String>> {
        let node = zed::node_binary_path()?;
        let output = zed::process::Command::new(node)
            .args([
                "--no-global-search-paths",
                "--eval",
                include_str!("project_binary.cjs"),
                "--",
                &worktree.root_path(),
                &self.name,
                self.binary_name,
                &pin.map(ToString::to_string).unwrap_or_default(),
            ])
            .env("NODE_OPTIONS", "")
            .env("NODE_PATH", "")
            .output()?;
        if output.status != Some(0) {
            return Err(String::from_utf8_lossy(&output.stderr).trim().to_owned());
        }
        let path = String::from_utf8(output.stdout).map_err(|error| error.to_string())?;
        Ok((!path.is_empty()).then_some(path))
    }

    fn modules_dir() -> Result<PathBuf> {
        std::env::current_dir()
            .map(|dir| dir.join("node_modules"))
            .map_err(|error| error.to_string())
    }

    fn remove_incomplete_package(&self, modules: &Path) -> Result<()> {
        let directory = modules.join(&self.name);
        match directory.metadata() {
            Ok(metadata) if metadata.is_dir() => {}
            Ok(_) => return Ok(()),
            Err(error) if error.kind() == ErrorKind::NotFound => return Ok(()),
            Err(error) => {
                return Err(format!(
                    "Could not inspect {}: {error}",
                    directory.display()
                ));
            }
        }

        let manifest = directory.join("package.json");
        let valid_manifest = match package_version(&manifest, &self.name) {
            Ok(_) => true,
            Err(error) if matches!(error.kind(), ErrorKind::NotFound | ErrorKind::InvalidData) => {
                false
            }
            Err(error) => return Err(format!("Could not read {}: {error}", manifest.display())),
        };
        let has_binary = executable_file_exists(&directory.join("lib").join(self.binary_name))?;
        if valid_manifest && has_binary {
            return Ok(());
        }

        // npm can report an existing package as up to date even if its executable is missing.
        match fs::remove_dir_all(&directory) {
            Ok(()) => Ok(()),
            Err(error) if error.kind() == ErrorKind::NotFound => Ok(()),
            Err(error) => Err(format!("Could not repair {}: {error}", directory.display())),
        }
    }

    fn read_installed(&self, modules: &Path) -> Result<ManagedBinary> {
        let read_version = |directory: &Path, name: &str| {
            let path = directory.join("package.json");
            package_version(&path, name)
                .map_err(|error| format!("Could not read {}: {error}", path.display()))
        };
        let package_dir = modules.join(&self.name);
        let native_version = read_version(&package_dir, &self.name)?;
        let version = read_version(&modules.join(PACKAGE_NAME), PACKAGE_NAME)?;
        if version.major < 7 || version != native_version {
            return Err(format!(
                "Installed TypeScript ({version}) and {} ({native_version}) must be matching native versions",
                self.name
            ));
        }
        let path = package_dir.join("lib").join(self.binary_name);
        if !executable_file_exists(&path)? {
            return Err(format!(
                "Native TypeScript executable is missing at {}",
                path.display()
            ));
        }
        Ok(ManagedBinary { path, version })
    }
}

fn executable_file_exists(path: &Path) -> Result<bool> {
    match path.metadata() {
        Ok(metadata) => Ok(metadata.is_file()),
        Err(error) if matches!(error.kind(), ErrorKind::NotFound | ErrorKind::NotADirectory) => {
            Ok(false)
        }
        Err(error) => Err(format!("Could not inspect {}: {error}", path.display())),
    }
}

fn package_version(path: &Path, package_name: &str) -> io::Result<Version> {
    let contents = fs::read(path)?;
    let manifest: Value = zed::serde_json::from_slice(&contents)
        .map_err(|error| io::Error::new(ErrorKind::InvalidData, error))?;
    if manifest.get("name").and_then(Value::as_str) != Some(package_name) {
        return Err(io::Error::new(
            ErrorKind::InvalidData,
            format!("Expected package name {package_name}"),
        ));
    }
    let version = manifest
        .get("version")
        .and_then(Value::as_str)
        .ok_or_else(|| io::Error::new(ErrorKind::InvalidData, "Missing or non-string version"))?;
    version
        .parse()
        .map_err(|error| io::Error::new(ErrorKind::InvalidData, error))
}

trait Npm {
    fn latest_version(&self) -> Result<String>;
    fn install(&self, target: &str) -> Result<()>;
    fn set_status(&self, status: Status);
}

struct ZedNpm<'a>(&'a LanguageServerId);

impl Npm for ZedNpm<'_> {
    fn latest_version(&self) -> Result<String> {
        zed::npm_package_latest_version(PACKAGE_NAME)
    }

    fn install(&self, target: &str) -> Result<()> {
        zed::npm_install_package(PACKAGE_NAME, target)
    }

    fn set_status(&self, status: Status) {
        zed::set_language_server_installation_status(self.0, &status);
    }
}

impl BinaryResolver {
    pub(crate) fn command(
        &mut self,
        id: &LanguageServerId,
        worktree: &Worktree,
        binary: CommandSettings,
        package_version: Option<&str>,
        prefer_project_binary: bool,
    ) -> Result<zed::Command> {
        let command = if let Some(path) = binary.path {
            worktree.which(&path).ok_or_else(|| {
                format!("TypeScript binary.path is not executable or could not be found: {path}")
            })?
        } else {
            self.resolve(id, worktree, package_version, prefer_project_binary)?
        };
        zed::set_language_server_installation_status(id, &Status::None);
        Ok(zed::Command {
            command,
            args: binary
                .arguments
                .unwrap_or_else(|| DEFAULT_ARGUMENTS.map(str::to_owned).to_vec()),
            env: binary.env.unwrap_or_default().into_iter().collect(),
        })
    }

    fn resolve(
        &mut self,
        id: &LanguageServerId,
        worktree: &Worktree,
        selector: Option<&str>,
        prefer_project_binary: bool,
    ) -> Result<String> {
        let platform = PlatformPackage::current()?;
        let pin = selector.and_then(|version| version.parse::<Version>().ok());
        if let Some(version) = &pin
            && version.major < 7
        {
            return Err(format!(
                "TypeScript {version} has no native language server. Set package_version to 7.0.0 or newer."
            ));
        }
        if prefer_project_binary && (selector.is_none() || pin.is_some()) {
            match platform.project_binary(worktree, pin.as_ref()) {
                Ok(Some(path)) => return Ok(path),
                Ok(None) => {}
                Err(error) => eprintln!("TypeScript project binary lookup failed: {error}"),
            }
        }

        let npm = ZedNpm(id);
        let result = PlatformPackage::modules_dir().and_then(|modules| {
            self.resolve_managed(&platform, &modules, selector, pin.as_ref(), &npm)
        });
        if let Err(error) = &result {
            npm.set_status(Status::Failed(error.clone()));
        }
        result.map(|path| path.display().to_string())
    }

    fn resolve_managed(
        &mut self,
        platform: &PlatformPackage,
        modules: &Path,
        selector: Option<&str>,
        pin: Option<&Version>,
        npm: &impl Npm,
    ) -> Result<PathBuf> {
        let reusable_version = if selector.is_none() {
            self.checked_latest.as_ref()
        } else {
            pin
        };
        let installed = match platform.read_installed(modules) {
            Ok(binary) if reusable_version == Some(&binary.version) => return Ok(binary.path),
            installed => installed,
        };

        npm.set_status(Status::CheckingForUpdate);
        let (target, lookup_error) = match selector {
            Some(selector) => (selector.to_owned(), None),
            None => match npm.latest_version() {
                Ok(version) => (version, None),
                Err(error) => ("latest".into(), Some(error)),
            },
        };
        let target_version = target.parse::<Version>().ok();
        if let Ok(binary) = installed {
            if target_version.as_ref() == Some(&binary.version) {
                if selector.is_none() {
                    self.checked_latest = Some(binary.version);
                }
                return Ok(binary.path);
            }
            if let Some(error) = &lookup_error {
                eprintln!(
                    "TypeScript update check failed, using installed {}: {error}",
                    binary.version
                );
                return Ok(binary.path);
            }
        }

        let with_lookup_error = |error| match &lookup_error {
            Some(lookup_error) => {
                format!("Could not check the latest TypeScript version ({lookup_error}). {error}")
            }
            None => error,
        };
        npm.set_status(Status::Downloading);
        platform
            .remove_incomplete_package(modules)
            .map_err(with_lookup_error)?;
        match install(platform, modules, &target, target_version.as_ref(), npm) {
            Ok(binary) => {
                if selector.is_none() {
                    self.checked_latest = Some(binary.version);
                }
                Ok(binary.path)
            }
            Err(error) => {
                let error = with_lookup_error(error);
                if selector.is_some() {
                    return Err(error);
                }
                match platform.read_installed(modules) {
                    Ok(binary) => {
                        eprintln!("{error} Using installed TypeScript {}.", binary.version);
                        Ok(binary.path)
                    }
                    Err(installed_error) => Err(format!(
                        "{error} Could not use the managed TypeScript installation: {installed_error}"
                    )),
                }
            }
        }
    }
}

fn install(
    platform: &PlatformPackage,
    modules: &Path,
    target: &str,
    exact: Option<&Version>,
    npm: &impl Npm,
) -> Result<ManagedBinary> {
    npm.install(target)
        .map_err(|error| format!("Could not install TypeScript {target}: {error}"))?;
    let binary = platform.read_installed(modules).map_err(|error| {
        format!(
            "TypeScript {target} was installed but its native executable is unavailable: {error}"
        )
    })?;
    if let Some(exact) = exact
        && exact != &binary.version
    {
        return Err(format!(
            "Requested TypeScript {exact}, but npm installed {}",
            binary.version
        ));
    }
    Ok(binary)
}
