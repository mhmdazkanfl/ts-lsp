use std::{
    fs,
    path::{Path, PathBuf},
};

use semver::Version;
use zed_extension_api::{self as zed, LanguageServerId, LanguageServerInstallationStatus, Result};
use zed_extension_api::{Worktree, serde_json::Value, settings::CommandSettings};

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

    fn installed(&self) -> Result<ManagedBinary> {
        self.read_installed(&Self::modules_dir()?)
    }

    fn remove_incomplete_package(&self, modules: &Path) -> Result<()> {
        let directory = modules.join(&self.name);
        if directory.is_dir()
            && (!directory.join("lib").join(self.binary_name).is_file()
                || package_version(&directory.join("package.json"), &self.name).is_err())
        {
            // npm can report an existing package as up to date even if its executable is missing.
            fs::remove_dir_all(&directory)
                .map_err(|error| format!("Could not repair {}: {error}", directory.display()))?;
        }
        Ok(())
    }

    fn read_installed(&self, modules: &Path) -> Result<ManagedBinary> {
        let package_dir = modules.join(&self.name);
        let native_version = package_version(&package_dir.join("package.json"), &self.name)?;
        let version = package_version(
            &modules.join(PACKAGE_NAME).join("package.json"),
            PACKAGE_NAME,
        )?;
        if version.major < 7 || version != native_version {
            return Err(format!(
                "Installed TypeScript ({version}) and {} ({native_version}) must be matching native versions",
                self.name
            ));
        }
        let path = package_dir.join("lib").join(self.binary_name);
        if !path.is_file() {
            return Err(format!(
                "Native TypeScript executable is missing at {}",
                path.display()
            ));
        }
        Ok(ManagedBinary { path, version })
    }
}

fn package_version(path: &Path, package_name: &str) -> Result<Version> {
    let contents =
        fs::read_to_string(path).map_err(|error| format!("{}: {error}", path.display()))?;
    let manifest: Value = zed::serde_json::from_str(&contents)
        .map_err(|error| format!("{}: {error}", path.display()))?;
    if manifest.get("name").and_then(Value::as_str) != Some(package_name) {
        return Err(format!("Expected {package_name} at {}", path.display()));
    }
    manifest
        .get("version")
        .and_then(Value::as_str)
        .and_then(|version| version.parse().ok())
        .ok_or_else(|| format!("Invalid version in {}", path.display()))
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
        zed::set_language_server_installation_status(id, &LanguageServerInstallationStatus::None);
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
        use LanguageServerInstallationStatus as Status;

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

        let installed = platform.installed().ok();
        let reusable_version = if selector.is_none() {
            self.checked_latest.as_ref()
        } else {
            pin.as_ref()
        };
        if let Some(binary) = installed
            .as_ref()
            .filter(|binary| reusable_version == Some(&binary.version))
        {
            return Ok(binary.path.display().to_string());
        }

        zed::set_language_server_installation_status(id, &Status::CheckingForUpdate);
        let (target, lookup_error) = match selector {
            Some(selector) => (selector.to_owned(), None),
            None => match zed::npm_package_latest_version(PACKAGE_NAME) {
                Ok(version) => (version, None),
                Err(error) => ("latest".into(), Some(error)),
            },
        };
        let target_version = target.parse::<Version>().ok();
        if let Some(binary) = installed {
            if target_version.as_ref() == Some(&binary.version) {
                if selector.is_none() {
                    self.checked_latest = Some(binary.version);
                }
                return Ok(binary.path.display().to_string());
            }
            if let Some(error) = &lookup_error {
                eprintln!(
                    "TypeScript update check failed, using installed {}: {error}",
                    binary.version
                );
                return Ok(binary.path.display().to_string());
            }
        }

        zed::set_language_server_installation_status(id, &Status::Downloading);
        match install(&platform, &target, target_version.as_ref()) {
            Ok(binary) => {
                if selector.is_none() {
                    self.checked_latest = Some(binary.version);
                }
                Ok(binary.path.display().to_string())
            }
            Err(error) => {
                let error = match lookup_error {
                    Some(lookup_error) => format!(
                        "Could not check the latest TypeScript version ({lookup_error}). {error}"
                    ),
                    None => error,
                };
                if selector.is_none()
                    && let Ok(binary) = platform.installed()
                {
                    eprintln!("{error} Using installed TypeScript {}.", binary.version);
                    return Ok(binary.path.display().to_string());
                }
                zed::set_language_server_installation_status(id, &Status::Failed(error.clone()));
                Err(error)
            }
        }
    }
}

fn install(
    platform: &PlatformPackage,
    target: &str,
    exact: Option<&Version>,
) -> Result<ManagedBinary> {
    platform.remove_incomplete_package(&PlatformPackage::modules_dir()?)?;
    zed::npm_install_package(PACKAGE_NAME, target)
        .map_err(|error| format!("Could not install TypeScript {target}: {error}"))?;
    let binary = platform.installed().map_err(|error| {
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
