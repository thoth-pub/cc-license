use crate::error::ParseError;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub(crate) enum Rights {
    By,
    BySa,
    ByNd,
    ByNc,
    ByNcSa,
    ByNcNd,
    Zero,
}

impl Rights {
    pub(crate) fn full_text(&self) -> &str {
        match self {
            Rights::By => "Attribution",
            Rights::BySa => "Attribution-ShareAlike",
            Rights::ByNd => "Attribution-NoDerivatives",
            Rights::ByNc => "Attribution-NonCommercial",
            Rights::ByNcSa => "Attribution-NonCommercial-ShareAlike",
            Rights::ByNcNd => "Attribution-NonCommercial-NoDerivatives",
            Rights::Zero => "CC0",
        }
    }
}

impl fmt::Display for Rights {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rights = match self {
            Rights::By => "CC BY",
            Rights::BySa => "CC BY-SA",
            Rights::ByNd => "CC BY-ND",
            Rights::ByNc => "CC BY-NC",
            Rights::ByNcSa => "CC BY-NC-SA",
            Rights::ByNcNd => "CC BY-NC-ND",
            Rights::Zero => "CC0",
        };
        write!(f, "{}", rights)
    }
}

impl FromStr for Rights {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "by" => Ok(Rights::By),
            "by-sa" => Ok(Rights::BySa),
            "by-nd" => Ok(Rights::ByNd),
            "by-nc" => Ok(Rights::ByNc),
            "by-nc-sa" => Ok(Rights::ByNcSa),
            "by-nc-nd" => Ok(Rights::ByNcNd),
            "zero" => Ok(Rights::Zero),
            &_ => Err(ParseError::InvalidRights),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_to_string() {
        assert_eq!(format!("{}", Rights::By), "CC BY".to_string());
        assert_eq!(format!("{}", Rights::BySa), "CC BY-SA".to_string());
        assert_eq!(format!("{}", Rights::ByNd), "CC BY-ND".to_string());
        assert_eq!(format!("{}", Rights::ByNc), "CC BY-NC".to_string());
        assert_eq!(format!("{}", Rights::ByNcSa), "CC BY-NC-SA".to_string());
        assert_eq!(format!("{}", Rights::ByNcNd), "CC BY-NC-ND".to_string());
        assert_eq!(format!("{}", Rights::Zero), "CC0".to_string());
    }

    #[test]
    fn test_from_string() {
        assert_eq!(Rights::from_str("by").unwrap(), Rights::By);
        assert_eq!(Rights::from_str("by-sa").unwrap(), Rights::BySa);
        assert_eq!(Rights::from_str("by-nd").unwrap(), Rights::ByNd);
        assert_eq!(Rights::from_str("by-nc").unwrap(), Rights::ByNc);
        assert_eq!(Rights::from_str("by-nc-sa").unwrap(), Rights::ByNcSa);
        assert_eq!(Rights::from_str("by-nc-nd").unwrap(), Rights::ByNcNd);
        assert_eq!(Rights::from_str("zero").unwrap(), Rights::Zero);

        assert!(Rights::from_str("CC by").is_err());
        assert!(Rights::from_str("cc By").is_err());
        assert!(Rights::from_str("Creative Commons BY").is_err());
    }

    #[test]
    fn test_full_text() {
        assert_eq!(Rights::By.full_text(), "Attribution");
        assert_eq!(Rights::BySa.full_text(), "Attribution-ShareAlike");
        assert_eq!(Rights::ByNd.full_text(), "Attribution-NoDerivatives");
        assert_eq!(Rights::ByNc.full_text(), "Attribution-NonCommercial");
        assert_eq!(
            Rights::ByNcSa.full_text(),
            "Attribution-NonCommercial-ShareAlike"
        );
        assert_eq!(
            Rights::ByNcNd.full_text(),
            "Attribution-NonCommercial-NoDerivatives"
        );
        assert_eq!(Rights::Zero.full_text(), "CC0");
    }
}
