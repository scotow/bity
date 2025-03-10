//! SI prefixed packet-rate parsing and formatting.
//!
//! # Examples
//!
//! ```
//! use bity::pps::{format, parse};
//!
//! assert_eq!(parse("12.3kp/s").unwrap(), 12_300);
//! assert_eq!(parse("0.12kpps").unwrap(), 120);
//! assert_eq!(parse("12p").unwrap(), 12);
//! assert_eq!(parse("12").unwrap(), 12);
//!
//! assert_eq!(format(1_234), "1.23kp/s");
//! assert_eq!(format(123_456), "123.45kp/s");
//! assert_eq!(format(12_345_678), "12.34Mp/s");
//! ```
//!
//! # Serde
//!
//! Enabling the `serde` feature allows the use of `#[serde(serialize_with =
//! "bity::pps::serialize")]`, `#[serde(deserialize_with =
//! "bity::pps::deserialize")]` and `#[serde(with = "bity::pps")]` attributes.
//!
//! ```
//! # use indoc::indoc;
//! # use serde::{Deserialize, Serialize};
//! #
//! #[derive(Serialize, Deserialize, PartialEq, Debug)]
//! #[serde(rename_all = "kebab-case")]
//! struct Config {
//!     #[serde(with = "bity::pps")]
//!     bandwidth: u64,
//!     #[serde(with = "bity::pps")]
//!     nic: u64,
//!     #[serde(with = "bity::pps")]
//!     highest: u64,
//! }
//!
//! assert_eq!(
//!     toml::from_str::<Config>(
//!         r#"
//!         bandwidth = "5.1Mp/s"
//!         nic = "180kp"
//!         highest = 12_000
//!         "#
//!     )
//!     .unwrap(),
//!     Config {
//!         bandwidth: 5_100_000,
//!         nic: 180_000,
//!         highest: 12_000,
//!     }
//! );
//!
//! assert_eq!(
//!     toml::to_string(&Config {
//!         bandwidth: 5_100_000,
//!         nic: 180_000,
//!         highest: 12_000,
//!     })
//!     .unwrap(),
//!     indoc! {
//!         r#"
//!         bandwidth = "5.1Mp/s"
//!         nic = "180kp/s"
//!         highest = "12kp/s"
//!         "#
//!     }
//! );
//! ```

use crate::{error::Error, packet};

/// Parse a packet-rate SI prefixed string into a number.
///
/// This is equivalent to calling `packet::parse(strip_per_second(input))`.
///
/// Refer to [`si::parse`](crate::si::parse) and [`packet::parse`] to learn the
/// rules that apply.
///
/// # Examples
/// ```
/// use bity::pps::parse;
///
/// assert_eq!(parse("12p/s").unwrap(), 12);
/// assert_eq!(parse("12pps").unwrap(), 12);
/// assert_eq!(parse("12.345kp/s").unwrap(), 12_345);
/// assert_eq!(parse("12.345kpps").unwrap(), 12_345);
/// assert_eq!(parse("12p").unwrap(), 12);
/// assert_eq!(parse("12").unwrap(), 12);
/// ```
pub fn parse(input: &str) -> Result<u64, Error<'_>> {
    packet::parse(crate::strip_per_second(input))
}

/// Format an integer into a packet-rate SI prefixed string.
///
/// This is equivalent to calling `format!("{}/s", packet::format(input))`.
///
/// Refer to [`si::format`](crate::si::format) and [`packet::format`] to learn
/// the rules that apply.
///
/// # Examples
/// ```
/// use bity::pps::format;
///
/// assert_eq!(format(12), "12p/s");
/// assert_eq!(format(1_234), "1.23kp/s");
/// assert_eq!(format(12_000), "12kp/s");
/// ```
pub fn format(input: u64) -> String {
    format!("{}/s", packet::format(input))
}

#[cfg(feature = "serde")]
crate::impl_serde!(
    ser:
    /// Serialize a given `u64` into a SI prefixed packet-rate string.
    ///
    /// Enabling the `serde` feature allows the use of `#[serde(serialize_with = "bity::pps::serialize")]` and `#[serde(with = "bity::pps")]` attributes.
    ///
    /// ```
    /// # use indoc::indoc;
    /// # use serde::Serialize;
    /// #
    /// #[derive(Serialize)]
    /// #[serde(rename_all = "kebab-case")]
    /// struct Config {
    ///     #[serde(with = "bity::pps")]
    ///     bandwidth: u64,
    ///     #[serde(serialize_with = "bity::pps::serialize")]
    ///     nic: u64,
    /// }
    ///
    /// assert_eq!(
    ///     toml::to_string(&Config {
    ///         bandwidth: 5_100_000,
    ///         nic: 180_000,
    ///     })
    ///     .unwrap(),
    ///     indoc! {
    ///         r#"
    ///         bandwidth = "5.1Mp/s"
    ///         nic = "180kp/s"
    ///         "#
    ///     }
    /// );
    /// ```
    de:
    /// Deserialize a given integer or SI prefixed packet-rate string into an `u64`.
    ///
    /// Enabling the `serde` feature allows the use of `#[serde(deserialize_with = "bity::pps::deserialize")]` and `#[serde(with = "bity::pps")]` attributes.
    ///
    /// ```
    /// # use indoc::indoc;
    /// # use serde::Deserialize;
    /// #
    /// #[derive(Deserialize, PartialEq, Debug)]
    /// #[serde(rename_all = "kebab-case")]
    /// struct Config {
    ///     #[serde(with = "bity::pps")]
    ///     bandwidth: u64,
    ///     #[serde(deserialize_with = "bity::pps::deserialize")]
    ///     nic: u64,
    ///     #[serde(deserialize_with = "bity::pps::deserialize")]
    ///     highest: u64,
    /// }
    ///
    /// assert_eq!(
    ///     toml::from_str::<Config>(
    ///         r#"
    ///         bandwidth = "5.1Mp/s"
    ///         nic = "180kp"
    ///         highest = 12_000
    ///         "#
    ///     )
    ///     .unwrap(),
    ///     Config {
    ///         bandwidth: 5_100_000,
    ///         nic: 180_000,
    ///         highest: 12_000,
    ///     }
    /// );
    /// ```
);

#[cfg(test)]
mod tests {
    #[test]
    fn parse() {
        assert_eq!(super::parse("12p/s").unwrap(), 12);
        assert_eq!(super::parse("12pps").unwrap(), 12);
        assert_eq!(super::parse("12.345kp/s").unwrap(), 12_345);
        assert_eq!(super::parse("12.345kpps").unwrap(), 12_345);

        assert_eq!(super::parse("12ps").unwrap(), 12);
        assert_eq!(super::parse("12p").unwrap(), 12);
        assert_eq!(super::parse("12").unwrap(), 12);
    }

    #[test]
    fn format() {
        assert_eq!(super::format(123), "123p/s");
        assert_eq!(super::format(1_234), "1.23kp/s");
        assert_eq!(super::format(12_000), "12kp/s");
    }
}
