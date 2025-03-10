//! SI prefixed data-rate (byte) parsing and formatting.
//!
//! # Examples
//!
//! ```
//! use bity::byteps::{format, parse};
//!
//! assert_eq!(parse("12.3kB/s").unwrap(), 12_300);
//! assert_eq!(parse("80bps").unwrap(), 10);
//!
//! assert_eq!(format(1_234), "1.23kB/s");
//! assert_eq!(format(123_456), "123.45kB/s");
//! assert_eq!(format(12_345_678), "12.34MB/s");
//! ```
//!
//! # Serde
//!
//! Enabling the `serde` feature allows the use of `#[serde(serialize_with =
//! "bity::byteps::serialize")]`, `#[serde(deserialize_with =
//! "bity::byteps::deserialize")]` and `#[serde(with = "bity::byteps")]`
//! attributes.
//!
//! ```
//! # use indoc::indoc;
//! # use serde::{Deserialize, Serialize};
//! #
//! #[derive(Serialize, Deserialize, PartialEq, Debug)]
//! #[serde(rename_all = "kebab-case")]
//! struct Config {
//!     #[serde(with = "bity::byteps")]
//!     bandwidth: u64,
//!     #[serde(with = "bity::byteps")]
//!     nic: u64,
//!     #[serde(with = "bity::byteps")]
//!     highest: u64,
//! }
//!
//! assert_eq!(
//!     toml::from_str::<Config>(
//!         r#"
//!         bandwidth = "5.1MB/s"
//!         nic = "180kB"
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
//!         bandwidth = "5.1MB/s"
//!         nic = "180kB/s"
//!         highest = "12kB/s"
//!         "#
//!     }
//! );
//! ```

use crate::{byte, error::Error};

/// Parse a data-rate SI prefixed string into a number of bytes per second.
///
/// This is equivalent to calling `byte::parse(strip_per_second(input))`.
///
/// Refer to [`si::parse`](crate::si::parse) and [`byte::parse`] to learn the
/// rules that apply.
///
/// # Examples
/// ```
/// use bity::byteps::parse;
///
/// assert_eq!(parse("12B/s").unwrap(), 12);
/// assert_eq!(parse("80bps").unwrap(), 10);
/// assert_eq!(parse("12.345kB/s").unwrap(), 12_345);
/// assert_eq!(parse("12.345kBps").unwrap(), 12_345);
/// assert_eq!(parse("12B").unwrap(), 12);
/// assert_eq!(parse("12").unwrap(), 12);
/// ```
pub fn parse(input: &str) -> Result<u64, Error<'_>> {
    byte::parse(crate::strip_per_second(input))
}

/// Format an integer into a data-rate SI prefixed string (byte oriented).
///
/// This is equivalent to calling `format!("{}/s", byte::format(input))`.
///
/// Refer to [`si::format`](crate::si::format) and [`byte::format`] to learn the
/// rules that apply.
///
/// # Examples
/// ```
/// use bity::byteps::format;
///
/// assert_eq!(format(12), "12B/s");
/// assert_eq!(format(1_234), "1.23kB/s");
/// assert_eq!(format(12_000), "12kB/s");
/// ```
pub fn format(input: u64) -> String {
    format!("{}/s", byte::format(input))
}

#[cfg(feature = "serde")]
crate::impl_serde!(
    ser:
    /// Serialize a given `u64` into a SI prefixed data-rate string.
    ///
    /// Enabling the `serde` feature allows the use of `#[serde(serialize_with = "bity::byteps::serialize")]` and `#[serde(with = "bity::byteps")]` attributes.
    ///
    /// ```
    /// # use indoc::indoc;
    /// # use serde::Serialize;
    /// #
    /// #[derive(Serialize)]
    /// #[serde(rename_all = "kebab-case")]
    /// struct Config {
    ///     #[serde(with = "bity::byteps")]
    ///     bandwidth: u64,
    ///     #[serde(serialize_with = "bity::byteps::serialize")]
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
    ///         bandwidth = "5.1MB/s"
    ///         nic = "180kB/s"
    ///         "#
    ///     }
    /// );
    /// ```
    de:
    /// Deserialize a given integer or SI prefixed data-rate string into an `u64`.
    ///
    /// Enabling the `serde` feature allows the use of `#[serde(deserialize_with = "bity::byteps::deserialize")]` and `#[serde(with = "bity::byteps")]` attributes.
    ///
    /// ```
    /// # use indoc::indoc;
    /// # use serde::Deserialize;
    /// #
    /// #[derive(Deserialize, PartialEq, Debug)]
    /// #[serde(rename_all = "kebab-case")]
    /// struct Config {
    ///     #[serde(with = "bity::byteps")]
    ///     bandwidth: u64,
    ///     #[serde(deserialize_with = "bity::byteps::deserialize")]
    ///     nic: u64,
    ///     #[serde(deserialize_with = "bity::byteps::deserialize")]
    ///     highest: u64,
    /// }
    ///
    /// assert_eq!(
    ///     toml::from_str::<Config>(
    ///         r#"
    ///         bandwidth = "5.1MB/s"
    ///         nic = "180kB"
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
        assert_eq!(super::parse("12B/s").unwrap(), 12);
        assert_eq!(super::parse("12Bps").unwrap(), 12);
        assert_eq!(super::parse("12.345kB/s").unwrap(), 12_345);
        assert_eq!(super::parse("12.345kBps").unwrap(), 12_345);

        assert_eq!(super::parse("12ps").unwrap(), 12);
        assert_eq!(super::parse("12B").unwrap(), 12);
        assert_eq!(super::parse("12").unwrap(), 12);
    }

    #[test]
    fn format() {
        assert_eq!(super::format(123), "123B/s");
        assert_eq!(super::format(1_234), "1.23kB/s");
        assert_eq!(super::format(12_000), "12kB/s");
    }
}
