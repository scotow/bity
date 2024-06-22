//! SI prefixed data parsing and formatting.
//!
//! # Examples
//!
//! ```
//! use bity::bit::{format, parse};
//!
//! assert_eq!(parse("12.3kb").unwrap(), 12_300);
//! assert_eq!(parse("0.12kB").unwrap(), 120 * 8);
//!
//! assert_eq!(format(1_234), "1.23kb");
//! assert_eq!(format(123_456), "123.45kb");
//! assert_eq!(format(12_345_678), "12.34Mb");
//! ```
//!
//! # Serde
//!
//! Enabling the `serde` allows the use of `#[serde(serialize_with =
//! "bity::bit::serialize")]`, `#[serde(deserialize_with =
//! "bity::bit::deserialize")]` and `#[serde(with = "bity::bit")]` attributes.
//!
//! ```
//! use indoc::indoc;
//! use serde::{Deserialize, Serialize};
//!
//! #[derive(Serialize, Deserialize, PartialEq, Debug)]
//! #[serde(rename_all = "kebab-case")]
//! struct Configuration {
//!     #[serde(with = "bity::bit")]
//!     user_quota: u64,
//!     #[serde(with = "bity::bit")]
//!     max_size: u64,
//! }
//!
//! assert_eq!(
//!     toml::from_str::<Configuration>(indoc! {r#"
//!         user-quota = "1.5kb"
//!         max-size = 180
//!     "#})
//!     .unwrap(),
//!     Configuration {
//!         user_quota: 1_500,
//!         max_size: 180,
//!     }
//! );
//!
//! assert_eq!(
//!     toml::to_string(&Configuration {
//!         user_quota: 1_500,
//!         max_size: 180,
//!     })
//!     .unwrap(),
//!     indoc! {r#"
//!         user-quota = "1.5kb"
//!         max-size = "180b"
//!     "#}
//! );
//! ```

use crate::{error::Error, si};

/// Parse a data SI prefixed string into a number.
///
/// This is equivalent to colling `si::parse_with_additional_units(input,
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
/// This is equivalent to colling `format!("{}b", si::format(input))`.
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
    /// Enabling the `serde` allows the use of `#[serde(serialize_with = "bity::bit::serialize")]` and `#[serde(with = "bity::bit")]` attributes.
    ///
    /// ```
    /// use indoc::indoc;
    /// use serde::Serialize;
    ///
    /// #[derive(Serialize)]
    /// #[serde(rename_all = "kebab-case")]
    /// struct Configuration {
    ///     #[serde(serialize_with = "bity::bit::serialize")]
    ///     user_quota: u64,
    ///     #[serde(with = "bity::bit")]
    ///     max_size: u64,
    /// }
    ///
    /// assert_eq!(
    ///     toml::to_string(&Configuration {
    ///         user_quota: 1_500,
    ///         max_size: 180,
    ///     }).unwrap(),
    ///     indoc! {r#"
    ///         user-quota = "1.5kb"
    ///         max-size = "180b"
    ///     "#}
    /// );
    /// ```
    de:
    /// Deserialize a given integer or SI prefixed data string into an `u64`.
    ///
    /// Enabling the `serde` allows the use of `#[serde(deserialize_with = "bity::bit::deserialize")]` and `#[serde(with = "bity::bit")]` attributes.
    ///
    /// ```
    /// use indoc::indoc;
    /// use serde::Deserialize;
    ///
    /// #[derive(Deserialize, PartialEq, Debug)]
    /// #[serde(rename_all = "kebab-case")]
    /// struct Configuration {
    ///     #[serde(deserialize_with = "bity::bit::deserialize")]
    ///     user_quota: u64,
    ///     #[serde(with = "bity::bit")]
    ///     max_size: u64,
    /// }
    ///
    /// assert_eq!(
    ///     toml::from_str::<Configuration>(
    ///         indoc! {r#"
    ///             user-quota = "1.5kb"
    ///             max-size = 180
    ///         "#}
    ///     ).unwrap(),
    ///     Configuration {
    ///         user_quota: 1_500,
    ///         max_size: 180,
    ///     }
    /// );
    /// ```
);

#[cfg(test)]
mod tests {
    use crate::error::Error;

    #[test]
    fn parse() {
        assert_eq!(super::parse("12b").unwrap(), 12);
        assert_eq!(super::parse("12B").unwrap(), 96);
        assert_eq!(super::parse("12kb").unwrap(), 12_000);
        assert_eq!(super::parse("12.345kb").unwrap(), 12_345);
        assert_eq!(super::parse("0.12kb").unwrap(), 120);

        assert_eq!(super::parse("12.345kB").unwrap(), 98_760);
        assert_eq!(super::parse("12.3Mb").unwrap(), 12_300_000);
        assert_eq!(super::parse("12.3MB").unwrap(), 98_400_000);
        assert_eq!(super::parse("12.3Gb").unwrap(), 12_300_000_000);
        assert_eq!(super::parse("12.3GB").unwrap(), 98_400_000_000);
        assert_eq!(super::parse("12.3Tb").unwrap(), 12_300_000_000_000);
        assert_eq!(super::parse("12.3TB").unwrap(), 98_400_000_000_000);
        assert_eq!(super::parse("12.3Pb").unwrap(), 12_300_000_000_000_000);
        assert_eq!(super::parse("12.3PB").unwrap(), 98_400_000_000_000_000);

        // "Strange" fractions.
        assert_eq!(super::parse("0.2").unwrap(), 0); // Less than a bit.
        assert_eq!(super::parse("0.125B").unwrap(), 1); // One bit from a byte.
        assert_eq!(super::parse("0.3B").unwrap(), 2); // Round to previous bit.
        assert_eq!(super::parse("12.3B").unwrap(), 98); // Round to previous bit.
        assert_eq!(super::parse("12.34B").unwrap(), 98); // Round to previous bit.
        assert_eq!(super::parse("012.340kb").unwrap(), 12_340); // Unused zeroes.
        assert_eq!(super::parse("12.3456kb").unwrap(), 12_345); // Overflowing fraction.
        assert_eq!(super::parse("12.3456kB").unwrap(), 98_764); // Byte rounding.
        assert_eq!(super::parse("12.34567kB").unwrap(), 98_765); // Byte rounding.
        assert_eq!(super::parse(".5kb").unwrap(), 500); // Missing integer.
        assert_eq!(super::parse("5.kb").unwrap(), 5_000); // Missing fraction.

        // Missing units.
        assert_eq!(super::parse("12k").unwrap(), 12_000);
        assert_eq!(super::parse("12").unwrap(), 12);

        // Additional spaces.
        assert_eq!(super::parse(" 12kb").unwrap(), 12_000);
        assert_eq!(super::parse("12kb ").unwrap(), 12_000);
        assert_eq!(super::parse("12 kb").unwrap(), 12_000);

        // Invalid units.
        assert!(matches!(super::parse("12Q"), Err(Error::InvalidUnit("Q"))));
        assert!(matches!(super::parse("12kk"), Err(Error::InvalidUnit("kk"))));
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
