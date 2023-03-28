use crate::error::ParseError;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub(crate) enum Version {
    One,
    Two,
    TwoFive,
    Three,
    Four,
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let version = match self {
            Version::One => "1.0",
            Version::Two => "2.0",
            Version::TwoFive => "2.5",
            Version::Three => "3.0",
            Version::Four => "4.0",
        };
        write!(f, "{}", version)
    }
}

impl FromStr for Version {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1.0" => Ok(Version::One),
            "2.0" => Ok(Version::Two),
            "2.5" => Ok(Version::TwoFive),
            "3.0" => Ok(Version::Three),
            "4.0" => Ok(Version::Four),
            &_ => Err(ParseError::InvalidVersion),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_to_string() {
        assert_eq!(format!("{}", Version::One), "1.0".to_string());
        assert_eq!(format!("{}", Version::Two), "2.0".to_string());
        assert_eq!(format!("{}", Version::TwoFive), "2.5".to_string());
        assert_eq!(format!("{}", Version::Three), "3.0".to_string());
        assert_eq!(format!("{}", Version::Four), "4.0".to_string());
    }

    #[test]
    fn test_from_string() {
        assert_eq!(Version::from_str("1.0").unwrap(), Version::One);
        assert_eq!(Version::from_str("2.0").unwrap(), Version::Two);
        assert_eq!(Version::from_str("2.5").unwrap(), Version::TwoFive);
        assert_eq!(Version::from_str("3.0").unwrap(), Version::Three);
        assert_eq!(Version::from_str("4.0").unwrap(), Version::Four);

        assert!(Version::from_str("1").is_err());
        assert!(Version::from_str("2").is_err());
        assert!(Version::from_str("4.5").is_err());
    }
}
