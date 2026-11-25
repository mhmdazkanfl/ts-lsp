pub fn is_supported(version: &semver::Version) -> bool {
    version.major >= 7 && version.pre.is_empty()
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
    fn installs_only_newer_versions_or_repairs_an_empty_install() {
        let installed = semver::Version::parse("7.0.2").unwrap();
        for (available, expected) in [
            ("7.0.1", false),
            ("7.0.2", false),
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
