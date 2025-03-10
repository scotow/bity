//! SI prefixed data (byte) parsing and formatting.
//!
//! # Examples
//!
//! ```
//! use bity::byte::{format, parse};
//!
//! assert_eq!(parse("12.3kB").unwrap(), 12_300);
//! assert_eq!(parse("800b").unwrap(), 100);
//!
//! assert_eq!(format(1_234), "1.23kB");
//! assert_eq!(format(123_456), "123.45kB");
//! assert_eq!(format(12_345_678), "12.34MB");
//! ```
//!
//! # Serde
//!
//! Enabling the `serde` feature allows the use of `#[serde(serialize_with =
//! "bity::byte::serialize")]`, `#[serde(deserialize_with =
//! "bity::byte::deserialize")]` and `#[serde(with = "bity::byte")]` attributes.
//!
//! ```
//! # use indoc::indoc;
//! # use serde::{Deserialize, Serialize};
//! #
//! #[derive(Serialize, Deserialize, PartialEq, Debug)]
//! #[serde(rename_all = "kebab-case")]
//! struct Config {
//!     #[serde(with = "bity::byte")]
//!     user_quota: u64,
//!     #[serde(with = "bity::byte")]
//!     max_size: u64,
//! }
//!
//! assert_eq!(
//!     toml::from_str::<Config>(
//!         r#"
//!         user-quota = "1.5kB"
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
//!         user-quota = "1.5kB"
//!         max-size = "180B"
//!         "#
//!     }
//! );
//! ```

use crate::{error::Error, si};

/// Parse a data SI prefixed string into a number of bytes.
///
/// This is kinda equivalent to calling `si::parse_with_additional_units(input,
/// &[("b", 1), ("B", 8)]).map(|n| n / 8)`.
///
/// Refer to [`si::parse`] and [`si::parse_with_additional_units`] to learn the
/// rules that apply.
///
/// # Examples
/// ```
/// use bity::byte::parse;
///
/// assert_eq!(parse("12b").unwrap(), 1);
/// assert_eq!(parse("12B").unwrap(), 12);
/// assert_eq!(parse("12kB").unwrap(), 12_000);
/// assert_eq!(parse("12.345kB").unwrap(), 12_345);
/// assert_eq!(parse("0.12kB").unwrap(), 120);
/// assert_eq!(parse("12.3MB").unwrap(), 12_300_000);
/// ```
pub fn parse(input: &str) -> Result<u64, Error<'_>> {
    if input.contains('b') {
        si::parse_with_additional_units(input, &[("b", 1), ("B", 8)]).map(|n| n / 8)
    } else {
        si::parse_with_additional_units(input, &[("B", 1)])
    }
}

/// Format an integer into a data SI prefixed string (byte oriented).
///
/// This is equivalent to calling `format!("{}B", si::format(input))`.
///
/// Refer to [`si::format`] to learn the rules that apply.
///
/// # Examples
/// ```
/// use bity::byte::format;
///
/// assert_eq!(format(12), "12B");
/// assert_eq!(format(1_234), "1.23kB");
/// assert_eq!(format(12_000), "12kB");
/// ```
pub fn format(input: u64) -> String {
    format!("{}B", si::format(input))
}

#[cfg(feature = "serde")]
crate::impl_serde!(
    ser:
    /// Serialize a given `u64` into a SI prefixed data string.
    ///
    /// Enabling the `serde` feature allows the use of `#[serde(serialize_with = "bity::byte::serialize")]` and `#[serde(with = "bity::byte")]` attributes.
    ///
    /// ```
    /// # use indoc::indoc;
    /// # use serde::Serialize;
    /// #
    /// #[derive(Serialize)]
    /// #[serde(rename_all = "kebab-case")]
    /// struct Config {
    ///     #[serde(serialize_with = "bity::byte::serialize")]
    ///     user_quota: u64,
    ///     #[serde(with = "bity::byte")]
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
    ///         user-quota = "1.5kB"
    ///         max-size = "180B"
    ///         "#
    ///     }
    /// );
    /// ```
    de:
    /// Deserialize a given integer or SI prefixed data string into an `u64`.
    ///
    /// Enabling the `serde` feature allows the use of `#[serde(deserialize_with = "bity::byte::deserialize")]` and `#[serde(with = "bity::byte")]` attributes.
    ///
    /// ```
    /// # use indoc::indoc;
    /// # use serde::Deserialize;
    /// #
    /// #[derive(Deserialize, PartialEq, Debug)]
    /// #[serde(rename_all = "kebab-case")]
    /// struct Config {
    ///     #[serde(deserialize_with = "bity::byte::deserialize")]
    ///     user_quota: u64,
    ///     #[serde(with = "bity::byte")]
    ///     max_size: u64,
    /// }
    ///
    /// assert_eq!(
    ///     toml::from_str::<Config>(
    ///         r#"
    ///         user-quota = "1.5kB"
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
        assert_eq!(super::parse("5b").unwrap(), 0);
        assert_eq!(super::parse("12b").unwrap(), 1);
        assert_eq!(super::parse("80b").unwrap(), 10);
        assert_eq!(super::parse("12.345kb").unwrap(), 12_345 / 8);
        assert_eq!(super::parse("12.345kB").unwrap(), 12_345);
    }

    #[test]
    fn format() {
        assert_eq!(super::format(0), "0B");
        assert_eq!(super::format(1), "1B");
        assert_eq!(super::format(12), "12B");
        assert_eq!(super::format(1_234), "1.23kB");
        assert_eq!(super::format(12_000), "12kB");
    }
}
