//! SI prefixed data (bit) parsing and formatting.
//!
//! # Examples
//!
//! ```
//! use bity::bit::{format, parse};
//!
//! assert_eq!(parse("12.3kb").unwrap(), 12_300);
//! assert_eq!(parse("0.12kB").unwrap(), 120 * 8);
//! assert_eq!(parse("518").unwrap(), 518);
//!
//! assert_eq!(format(1_234), "1.23kb");
//! assert_eq!(format(12_345_678), "12.34Mb");
//! ```
//!
//! # Serde
//!
//! Enabling the `serde` feature allows the use of `#[serde(serialize_with =
//! "bity::bit::serialize")]`, `#[serde(deserialize_with =
//! "bity::bit::deserialize")]` and `#[serde(with = "bity::bit")]` attributes.
//!
//! ```
//! # use indoc::indoc;
//! # use serde::{Deserialize, Serialize};
//! #
//! #[derive(Serialize, Deserialize, PartialEq, Debug)]
//! #[serde(rename_all = "kebab-case")]
//! struct Config {
//!     #[serde(with = "bity::bit")]
//!     user_quota: u64,
//!     #[serde(with = "bity::bit")]
//!     max_size: u64,
//! }
//!
//! assert_eq!(
//!     toml::from_str::<Config>(
//!         r#"
//!         user-quota = "1.5kb"
//!         max-size = 180
//!         "#
//!     )
//!     .unwrap(),
//!     Config {
//!         user_quota: 1_500,
//!         max_size: 180,
//!     }
//! );
//!
//! assert_eq!(
//!     toml::to_string(&Config {
//!         user_quota: 1_500,
//!         max_size: 180,
//!     })
//!     .unwrap(),
//!     indoc! {
//!         r#"
//!         user-quota = "1.5kb"
//!         max-size = "180b"
//!         "#
//!     }
//! );
//! ```

use crate::{error::Error, si};

/// Parse a data SI prefixed string into a number of bits.
///
/// This is equivalent to calling `si::parse_with_additional_units(input,
/// &[("b", 1), ("B", 8)])`.
///
/// Refer to [`si::parse`] and [`si::parse_with_additional_units`] to learn the
/// rules that apply.
///
/// # Examples
/// ```
/// use bity::bit::parse;
///
/// assert_eq!(parse("12b").unwrap(), 12);
/// assert_eq!(parse("12B").unwrap(), 96);
/// assert_eq!(parse("12kb").unwrap(), 12_000);
/// assert_eq!(parse("12.345kb").unwrap(), 12_345);
/// assert_eq!(parse("0.12kb").unwrap(), 120);
/// assert_eq!(parse("12.345kB").unwrap(), 98_760);
/// assert_eq!(parse("12.3Mb").unwrap(), 12_300_000);
/// assert_eq!(parse("12.3MB").unwrap(), 98_400_000);
/// ```
pub fn parse(input: &str) -> Result<u64, Error<'_>> {
    si::parse_with_additional_units(input, &[("b", 1), ("B", 8)])
}

/// Format an integer into a data SI prefixed string (bit oriented).
///
/// This is equivalent to calling `format!("{}b", si::format(input))`.
///
/// Refer to [`si::format`] to learn the rules that apply.
///
/// # Examples
/// ```
/// use bity::bit::format;
///
/// assert_eq!(format(12), "12b");
/// assert_eq!(format(1_234), "1.23kb");
/// assert_eq!(format(12_000), "12kb");
/// ```
pub fn format(input: u64) -> String {
    format!("{}b", si::format(input))
}

#[cfg(feature = "serde")]
crate::impl_serde!(
    ser:
    /// Serialize a given `u64` into a SI prefixed data string.
    ///
    /// Enabling the `serde` feature allows the use of `#[serde(serialize_with = "bity::bit::serialize")]` and `#[serde(with = "bity::bit")]` attributes.
    ///
    /// ```
    /// # use indoc::indoc;
    /// # use serde::Serialize;
    /// #
    /// #[derive(Serialize)]
    /// #[serde(rename_all = "kebab-case")]
    /// struct Config {
    ///     #[serde(serialize_with = "bity::bit::serialize")]
    ///     user_quota: u64,
    ///     #[serde(with = "bity::bit")]
    ///     max_size: u64,
    /// }
    ///
    /// assert_eq!(
    ///     toml::to_string(&Config {
    ///         user_quota: 1_500,
    ///         max_size: 180,
    ///     })
    ///     .unwrap(),
    ///     indoc! {
    ///         r#"
    ///         user-quota = "1.5kb"
    ///         max-size = "180b"
    ///         "#
    ///     }
    /// );
    /// ```
    de:
    /// Deserialize a given integer or SI prefixed data string into an `u64`.
    ///
    /// Enabling the `serde` feature allows the use of `#[serde(deserialize_with = "bity::bit::deserialize")]` and `#[serde(with = "bity::bit")]` attributes.
    ///
    /// ```
    /// # use indoc::indoc;
    /// # use serde::Deserialize;
    /// #
    /// #[derive(Deserialize, PartialEq, Debug)]
    /// #[serde(rename_all = "kebab-case")]
    /// struct Config {
    ///     #[serde(deserialize_with = "bity::bit::deserialize")]
    ///     user_quota: u64,
    ///     #[serde(with = "bity::bit")]
    ///     max_size: u64,
    /// }
    ///
    /// assert_eq!(
    ///     toml::from_str::<Config>(
    ///         r#"
    ///         user-quota = "1.5kb"
    ///         max-size = 180
    ///         "#
    ///     )
    ///     .unwrap(),
    ///     Config {
    ///         user_quota: 1_500,
    ///         max_size: 180,
    ///     }
    /// );
    /// ```
);

#[cfg(test)]
mod tests {
    #[test]
    fn parse() {
        assert_eq!(super::parse("12b").unwrap(), 12);
        assert_eq!(super::parse("12B").unwrap(), 96);
        assert_eq!(super::parse("12kb").unwrap(), 12_000);
        assert_eq!(super::parse("12.345kb").unwrap(), 12_345);
        assert_eq!(super::parse("0.12kb").unwrap(), 120);
    }

    #[test]
    fn format() {
        assert_eq!(super::format(0), "0b");
        assert_eq!(super::format(1), "1b");
        assert_eq!(super::format(12), "12b");
        assert_eq!(super::format(1_234), "1.23kb");
        assert_eq!(super::format(12_000), "12kb");
    }
}
