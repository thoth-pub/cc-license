mod error;
mod nomenclature;
mod rights;
mod version;

pub use crate::error::ParseError;
use crate::nomenclature::Nomenclature;
use crate::rights::Rights;
use crate::version::Version;
use regex::Regex;
use std::str::FromStr;

const CC_REGEX: &str = r"^https?://(www\.)?creativecommons\.org/(licenses|publicdomain)/(?P<rights>[^/]+)/(?P<version>[^/]+)/?$";

#[derive(Debug, PartialEq)]
pub struct License {
    rights: Rights,
    version: Version,
}

impl License {
    /// Parse a Creative Commons license from a URL
    ///
    /// # Example
    ///
    /// ```rust
    /// # use cc_license::ParseError;
    /// use cc_license::License;
    ///
    /// # fn run() -> Result<(), ParseError> {    ///
    /// let license = License::from_url("https://creativecommons.org/licenses/by-nc-sa/4.0/")?;
    /// assert_eq!(license.to_string(), "Creative Commons Attribution-NonCommercial-ShareAlike 4.0 International license (CC BY-NC-SA 4.0).".to_string());
    /// # Ok(())
    /// # }
    /// # run().unwrap();
    /// ```
    pub fn from_url(url: &str) -> Result<Self, ParseError> {
        let re = Regex::new(CC_REGEX).unwrap();
        let captures = re.captures(url).ok_or(ParseError::InvalidUrl)?;
        let rights = captures
            .name("rights")
            .ok_or(ParseError::InvalidUrl)
            .and_then(|r| Rights::from_str(r.as_str()))?;
        let version = captures
            .name("version")
            .ok_or(ParseError::InvalidUrl)
            .and_then(|v| Version::from_str(v.as_str()))?;

        let license = License { rights, version };
        license.check()?;
        Ok(license)
    }

    /// Obtain the abbreviated rights string from a license
    ///
    /// # Example
    ///
    /// ```rust
    /// # use cc_license::ParseError;
    /// use cc_license::License;
    ///
    /// # fn run() -> Result<(), ParseError> {    ///
    /// let license = License::from_url("https://creativecommons.org/licenses/by/4.0/")?;
    /// assert_eq!(license.rights(), "CC BY".to_string());
    /// # Ok(())
    /// # }
    /// # run().unwrap();
    /// ```
    pub fn rights(&self) -> String {
        self.rights.to_string()
    }

    /// Obtain the rights string from a license
    ///
    /// # Example
    ///
    /// ```rust
    /// # use cc_license::ParseError;
    /// use cc_license::License;
    ///
    /// # fn run() -> Result<(), ParseError> {    ///
    /// let license = License::from_url("https://creativecommons.org/licenses/by-nc-sa/4.0/")?;
    /// assert_eq!(license.rights_full(), "Attribution-NonCommercial-ShareAlike".to_string());
    /// # Ok(())
    /// # }
    /// # run().unwrap();
    /// ```
    pub fn rights_full(&self) -> String {
        self.rights.full_text().to_string()
    }

    /// Obtain the version string from a license
    ///
    /// # Example
    ///
    /// ```rust
    /// # use cc_license::ParseError;
    /// use cc_license::License;
    ///
    /// # fn run() -> Result<(), ParseError> {    ///
    /// let license = License::from_url("https://creativecommons.org/licenses/by/4.0/")?;
    /// assert_eq!(license.version(), "4.0".to_string());
    /// # Ok(())
    /// # }
    /// # run().unwrap();
    /// ```
    pub fn version(&self) -> String {
        self.version.to_string()
    }

    /// Obtain the abbreviation of the license
    ///
    /// # Example
    ///
    /// ```rust
    /// # use cc_license::ParseError;
    /// use cc_license::License;
    ///
    /// # fn run() -> Result<(), ParseError> {    ///
    /// let license = License::from_url("https://creativecommons.org/licenses/by-nc/4.0/")?;
    /// assert_eq!(license.short(), "CC BY-NC 4.0".to_string());
    /// # Ok(())
    /// # }
    /// # run().unwrap();
    /// ```
    pub fn short(&self) -> String {
        format!("{} {}", self.rights, self.version)
    }

    fn check(&self) -> Result<(), ParseError> {
        if self.rights == Rights::Zero && self.version != Version::One {
            return Err(ParseError::InvalidPublicDomainVersion);
        }
        Ok(())
    }
}

impl From<&License> for Nomenclature {
    fn from(license: &License) -> Self {
        match license.rights {
            Rights::Zero => Nomenclature::Universal,
            _ => match license.version {
                Version::One => Nomenclature::Generic,
                Version::Two => Nomenclature::Generic,
                Version::TwoFive => Nomenclature::Generic,
                Version::Three => Nomenclature::Unported,
                Version::Four => Nomenclature::International,
            },
        }
    }
}

impl ToString for License {
    fn to_string(&self) -> String {
        format!(
            "Creative Commons {} {} {} license ({}).",
            self.rights_full(),
            self.version,
            Nomenclature::from(self),
            self.short(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_url() {
        assert_eq!(
            License::from_url("https://creativecommons.org/licenses/by/4.0/").unwrap(),
            License {
                rights: Rights::By,
                version: Version::Four,
            }
        );
        assert_eq!(
            License::from_url("https://creativecommons.org/licenses/by-nc/1.0/").unwrap(),
            License {
                rights: Rights::ByNc,
                version: Version::One,
            }
        );
        assert_eq!(
            License::from_url("http://creativecommons.org/licenses/by-nc-sa/4.0/").unwrap(),
            License {
                rights: Rights::ByNcSa,
                version: Version::Four,
            }
        );
        assert_eq!(
            License::from_url("https://creativecommons.org/licenses/by-nc-nd/3.0").unwrap(),
            License {
                rights: Rights::ByNcNd,
                version: Version::Three,
            }
        );
        assert_eq!(
            License::from_url("https://creativecommons.org/publicdomain/zero/1.0/").unwrap(),
            License {
                rights: Rights::Zero,
                version: Version::One,
            }
        );

        assert!(License::from_url("creativecommons.org/licenses/by/1.0/").is_err());
        assert!(License::from_url("https://creativecommons.org/licenses/by/").is_err());
        assert_eq!(
            License::from_url("https://creativecommons.org/licenses/attribution/4.0/"),
            Err(ParseError::InvalidRights)
        );
        assert_eq!(
            License::from_url("https://creativecommons.org/licenses/by/5.0/"),
            Err(ParseError::InvalidVersion)
        );
        assert_eq!(
            License::from_url("https://creativecommons.org/publicdomain/zero/2.0/"),
            Err(ParseError::InvalidPublicDomainVersion)
        );
    }

    #[test]
    fn test_to_string() {
        let mut test_license = License {
            rights: Rights::By,
            version: Version::One,
        };
        assert_eq!(
            test_license.to_string(),
            "Creative Commons Attribution 1.0 Generic license (CC BY 1.0).".to_string()
        );
        test_license = License {
            rights: Rights::By,
            version: Version::Two,
        };
        assert_eq!(
            test_license.to_string(),
            "Creative Commons Attribution 2.0 Generic license (CC BY 2.0).".to_string()
        );
        test_license = License {
            rights: Rights::By,
            version: Version::TwoFive,
        };
        assert_eq!(
            test_license.to_string(),
            "Creative Commons Attribution 2.5 Generic license (CC BY 2.5).".to_string()
        );
        test_license = License {
            rights: Rights::By,
            version: Version::Three,
        };
        assert_eq!(
            test_license.to_string(),
            "Creative Commons Attribution 3.0 Unported license (CC BY 3.0).".to_string()
        );
        test_license = License {
            rights: Rights::By,
            version: Version::Four,
        };
        assert_eq!(
            test_license.to_string(),
            "Creative Commons Attribution 4.0 International license (CC BY 4.0).".to_string()
        );
        test_license = License {
            rights: Rights::ByNc,
            version: Version::Four,
        };
        assert_eq!(
            test_license.to_string(),
            "Creative Commons Attribution-NonCommercial 4.0 International license (CC BY-NC 4.0)."
                .to_string()
        );
        test_license = License {
            rights: Rights::ByNd,
            version: Version::Four,
        };
        assert_eq!(
            test_license.to_string(),
            "Creative Commons Attribution-NoDerivatives 4.0 International license (CC BY-ND 4.0)."
                .to_string()
        );
        test_license = License {
            rights: Rights::BySa,
            version: Version::Four,
        };
        assert_eq!(
            test_license.to_string(),
            "Creative Commons Attribution-ShareAlike 4.0 International license (CC BY-SA 4.0)."
                .to_string()
        );
        test_license = License {
            rights: Rights::ByNcSa,
            version: Version::Four,
        };
        assert_eq!(test_license.to_string(), "Creative Commons Attribution-NonCommercial-ShareAlike 4.0 International license (CC BY-NC-SA 4.0).".to_string());
        test_license = License {
            rights: Rights::ByNcNd,
            version: Version::Four,
        };
        assert_eq!(test_license.to_string(), "Creative Commons Attribution-NonCommercial-NoDerivatives 4.0 International license (CC BY-NC-ND 4.0).".to_string());
        test_license = License {
            rights: Rights::Zero,
            version: Version::One,
        };
        assert_eq!(
            test_license.to_string(),
            "Creative Commons CC0 1.0 Universal license (CC0 1.0).".to_string()
        );
    }

    #[test]
    fn to_nomenclature() {
        let mut test_license = License {
            rights: Rights::By,
            version: Version::One,
        };
        assert_eq!(Nomenclature::from(&test_license), Nomenclature::Generic);
        test_license = License {
            rights: Rights::By,
            version: Version::Two,
        };
        assert_eq!(Nomenclature::from(&test_license), Nomenclature::Generic);
        test_license = License {
            rights: Rights::By,
            version: Version::TwoFive,
        };
        assert_eq!(Nomenclature::from(&test_license), Nomenclature::Generic);
        test_license = License {
            rights: Rights::By,
            version: Version::Three,
        };
        assert_eq!(Nomenclature::from(&test_license), Nomenclature::Unported);
        test_license = License {
            rights: Rights::By,
            version: Version::Four,
        };
        assert_eq!(
            Nomenclature::from(&test_license),
            Nomenclature::International
        );
        test_license = License {
            rights: Rights::Zero,
            version: Version::One,
        };
        assert_eq!(Nomenclature::from(&test_license), Nomenclature::Universal);
    }
}
