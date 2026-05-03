use core::fmt::{self, Error as FmtError};
use std::error::Error as StdError;

use serenity::Error as SerenityError;
use sqlx::Error as SqlxError;
use std::num::ParseIntError as StdParseIntError;
use strum::ParseError as StrumParseError;
use tracing_subscriber::filter::ParseError as TracingSubscriberParseError;

#[derive(Debug)]
pub struct HesperError(pub String);

impl fmt::Display for HesperError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

macro_rules! impl_from_for_custom_error {
    ( $( $error_type:ty ),* ) => {
        $(
            impl From<$error_type> for HesperError {
                fn from(err: $error_type) -> Self {
                    HesperError(err.to_string())
                }
            }
        )*
    };
}

impl_from_for_custom_error!(
    &str,
    String,
    Box<dyn StdError>,
    SqlxError,
    StrumParseError,
    SerenityError,
    FmtError,
    StdParseIntError,
    TracingSubscriberParseError
);
