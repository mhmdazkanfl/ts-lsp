mod cache;
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
    let cache = cache::UpdateCache::new(std::path::Path::new(""));
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    ensure_package(
        &package,
        &cache,
        &package::ZedNpm,
        now,
        report,
    )?;
    Ok(package.into_binary())
}

fn ensure_package(
    package: &package::PlatformPackage,
    cache: &cache::UpdateCache,
    npm: &impl package::Npm,
    now: u64,
    mut report: impl FnMut(z::LanguageServerInstallationStatus),
) -> z::Result<()> {
    let installed = package
        .installed_version()
        .filter(update::is_supported);
    if !update::check_due(
        now,
        cache.last_check(),
        installed.is_some(),
    ) {
        return Ok(());
    }

    if let Err(err) = cache.record_check(now) {
        eprintln!("Failed to record the TypeScript update check: {err}");
    }
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
