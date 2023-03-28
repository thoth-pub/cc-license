use std::error::Error;
use std::fmt;

macro_rules! errors {
    ($($name: ident => $description: expr,)+) => {
        /// Errors that can occur during parsing.
        #[derive(PartialEq, Eq, Clone, Copy, Debug)]
        pub enum ParseError {
            $(
                $name,
            )+
        }

        impl fmt::Display for ParseError {
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                match *self {
                    $(
                        ParseError::$name => fmt.write_str($description),
                    )+
                }
            }
        }
    }
}

impl Error for ParseError {}

errors! {
    InvalidUrl => "Invalid URL",
    InvalidRights => "Invalid rights string",
    InvalidVersion => "Invalid version string",
    InvalidPublicDomainVersion => "The version of CC0 licenses must be 1.0",
}
