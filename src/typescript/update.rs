const UPDATE_CHECK_INTERVAL_SECS: u64 = 24 * 60 * 60;

pub fn is_supported(version: &semver::Version) -> bool {
    version.major >= 7 && version.pre.is_empty()
}

pub fn check_due(now: u64, last_check: Option<u64>, has_installed_version: bool) -> bool {
    !has_installed_version
        || last_check
            .and_then(|last_check| now.checked_sub(last_check))
            .is_none_or(|elapsed| elapsed >= UPDATE_CHECK_INTERVAL_SECS)
}

pub fn should_install(installed: Option<&semver::Version>, available: &semver::Version) -> bool {
    installed.is_none_or(|installed| {
        installed
            .cmp_precedence(available)
            .is_lt()
    })
}

#[cfg(test)]
mod tests {
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

    #[test]
    fn checks_at_24_hours_and_without_a_usable_cache() {
        let last_check = 1_000_000;
        let interval = 24 * 60 * 60;
        assert!(!super::check_due(
            last_check + interval - 1,
            Some(last_check),
            true
        ));
        assert!(super::check_due(
            last_check + interval,
            Some(last_check),
            true
        ));
        assert!(super::check_due(
            last_check,
            Some(last_check),
            false
        ));
        assert!(super::check_due(last_check, None, true));
        assert!(super::check_due(
            last_check - 1,
            Some(last_check),
            true
        ));
    }

    #[test]
    fn installs_only_newer_versions_or_repairs_an_empty_install() {
        let installed = semver::Version::parse("7.0.2").unwrap();
        for (available, expected) in [
            ("7.0.1", false),
            ("7.0.2+build", false),
            ("7.0.3", true),
        ] {
            let available = semver::Version::parse(available).unwrap();
            assert_eq!(
                super::should_install(Some(&installed), &available),
                expected
            );
            assert!(super::should_install(None, &available));
        }
    }
}
