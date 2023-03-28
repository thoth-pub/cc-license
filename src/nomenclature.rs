use std::fmt;

#[derive(Debug, PartialEq)]
pub(crate) enum Nomenclature {
    Generic,
    Unported,
    International,
    Universal,
}

impl fmt::Display for Nomenclature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let nomenclature = match self {
            Nomenclature::Generic => "Generic",
            Nomenclature::Unported => "Unported",
            Nomenclature::International => "International",
            Nomenclature::Universal => "Universal",
        };
        write!(f, "{}", nomenclature)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_string() {
        assert_eq!(format!("{}", Nomenclature::Generic), "Generic".to_string());
        assert_eq!(
            format!("{}", Nomenclature::Unported),
            "Unported".to_string()
        );
        assert_eq!(
            format!("{}", Nomenclature::International),
            "International".to_string()
        );
        assert_eq!(
            format!("{}", Nomenclature::Universal),
            "Universal".to_string()
        );
    }
}
