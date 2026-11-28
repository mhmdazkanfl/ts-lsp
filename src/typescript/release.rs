const MINIMUM_MAJOR_VERSION: u64 = 7;

#[derive(Debug, PartialEq, Eq)]
pub(super) struct Release(semver::Version);

#[derive(Debug)]
pub(super) struct ReleaseError<'a>(ReleaseErrorKind<'a>);

#[derive(Debug)]
enum ReleaseErrorKind<'a> {
    Invalid {
        input: &'a str,
        error: semver::Error,
    },
    Unsupported(semver::Version),
}

impl ReleaseError<'_> {
    pub(super) fn for_package(self, name: &str) -> String {
        match self.0 {
            ReleaseErrorKind::Invalid {
                input,
                error,
            } => format!("Invalid version {input} for {name}: {error}"),
            ReleaseErrorKind::Unsupported(version) => format!(
                "{name}@{version} is unsupported. Expected a stable TypeScript version {MINIMUM_MAJOR_VERSION} or newer"
            ),
        }
    }
}

impl Release {
    pub(super) fn parse(input: &str) -> Result<Self, ReleaseError<'_>> {
        let version: semver::Version = input.parse().map_err(|error| {
            ReleaseError(ReleaseErrorKind::Invalid {
                input,
                error,
            })
        })?;
        if version.major < MINIMUM_MAJOR_VERSION || !version.pre.is_empty() {
            return Err(ReleaseError(
                ReleaseErrorKind::Unsupported(version),
            ));
        }
        Ok(Self(version))
    }

    // Build labels matter when validating a specific install, but never trigger an update.
    pub(super) fn requires_download(&self, installed: Option<&Self>) -> bool {
        installed.is_none_or(|installed| {
            self.0
                .cmp_precedence(&installed.0)
                .is_gt()
        })
    }
}

impl std::fmt::Display for Release {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, formatter)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn a_malformed_version_identifies_the_value_and_package_in_the_error() {
        let version = "latest";

        let result = super::Release::parse(version);

        let error = result
            .unwrap_err()
            .for_package("typescript-test");
        assert!(error.starts_with("Invalid version latest for typescript-test:"));
    }

    #[test]
    fn the_minimum_stable_release_is_supported() {
        let version = "7.0.0";

        let result = super::Release::parse(version);

        assert!(result.is_ok());
    }

    #[test]
    fn a_stable_release_below_the_minimum_is_unsupported() {
        let version = "6.9.9";

        let result = super::Release::parse(version);

        let error = result
            .unwrap_err()
            .for_package("typescript-test");
        assert!(error.starts_with("typescript-test@6.9.9 is unsupported."));
    }

    #[test]
    fn even_a_newer_major_prerelease_is_unsupported() {
        let version = "8.0.0-rc.1";

        let result = super::Release::parse(version);

        let error = result
            .unwrap_err()
            .for_package("typescript-test");
        assert!(error.starts_with("typescript-test@8.0.0-rc.1 is unsupported."));
    }

    #[test]
    fn a_stable_release_can_include_a_build_label() {
        let version = "7.0.2+build.4";

        let result = super::Release::parse(version);

        assert_eq!(
            result.unwrap().to_string(),
            "7.0.2+build.4"
        );
    }

    #[test]
    fn a_first_install_requires_a_download() {
        let available = super::Release::parse("7.0.2").unwrap();

        let download = available.requires_download(None);

        assert!(download);
    }

    macro_rules! download_cases {
        ($($test:ident: ($installed:literal, $available:literal) => $expected:literal,)*) => {
            $(
                #[test]
                fn $test() {
                    let installed = super::Release::parse($installed).unwrap();
                    let available = super::Release::parse($available).unwrap();

                    let download = available.requires_download(Some(&installed));

                    assert_eq!(download, $expected);
                }
            )*
        };
    }

    download_cases! {
        a_newer_patch_requires_a_download: ("7.0.1", "7.0.2") => true,
        a_newer_minor_requires_a_download: ("7.1.9", "7.2.0") => true,
        a_newer_major_requires_a_download: ("7.9.9", "8.0.0") => true,
        the_same_release_requires_no_download: ("7.0.2", "7.0.2") => false,
        an_older_release_requires_no_download: ("7.0.2", "7.0.1") => false,
        a_build_label_change_requires_no_download: ("7.0.2+a", "7.0.2+z") => false,
    }
}
