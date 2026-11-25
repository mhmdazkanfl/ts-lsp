mod package;
mod update;

#[cfg(test)]
mod tests;

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
