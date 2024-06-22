//! SI prefixed packets count parsing and formatting.
//!
//! # Examples
//!
//! ```
//! use bity::packet::{format, parse};
//!
//! assert_eq!(parse("12.3kp").unwrap(), 12_300);
//! assert_eq!(parse("0.12kp").unwrap(), 120);
//!
//! assert_eq!(format(1_234), "1.23kp");
//! assert_eq!(format(123_456), "123.45kp");
//! assert_eq!(format(12_345_678), "12.34Mp");
//! ```
//!
//! # Serde
//!
//! Enabling the `serde` allows the use of `#[serde(serialize_with =
//! "bity::packet::serialize")]`, `#[serde(deserialize_with =
//! "bity::packet::deserialize")]` and `#[serde(with = "bity::packet")]`
//! attributes.
//!
//! ```
//! use indoc::indoc;
//! use serde::{Deserialize, Serialize};
//!
//! #[derive(Serialize, Deserialize, PartialEq, Debug)]
//! #[serde(rename_all = "kebab-case")]
//! struct Configuration {
//!     #[serde(with = "bity::packet")]
//!     monthly_usage: u64,
//!     #[serde(with = "bity::packet")]
//!     remaining: u64,
//! }
//!
//! assert_eq!(
//!     toml::from_str::<Configuration>(indoc! {r#"
//!         monthly-usage = "1.5kp"
//!         remaining = 180
//!     "#})
//!     .unwrap(),
//!     Configuration {
//!         monthly_usage: 1_500,
//!         remaining: 180,
//!     }
//! );
//!
//! assert_eq!(
//!     toml::to_string(&Configuration {
//!         monthly_usage: 1_500,
//!         remaining: 180,
//!     })
//!     .unwrap(),
//!     indoc! {r#"
//!         monthly-usage = "1.5kp"
//!         remaining = "180p"
//!     "#}
//! );
//! ```

use crate::{si, Error};

/// Parse a packet count SI prefixed string into a number.
///
/// This is equivalent to colling `si::parse_with_additional_units(input,
/// &[("p", 1)])`.
///
/// Refer to [`si::parse`] and [`si::parse_with_additional_units`] to learn the
/// rules that apply.
///
/// # Examples
/// ```
/// use bity::packet::parse;
///
/// assert_eq!(parse("12p").unwrap(), 12);
/// assert_eq!(parse("12.345kp").unwrap(), 12_345);
/// assert_eq!(parse("12").unwrap(), 12);
/// ```
pub fn parse(input: &str) -> Result<u64, Error<'_>> {
    si::parse_with_additional_units(input, &[("p", 1)])
}

/// Format an integer into a packet count SI prefixed string.
///
/// This is equivalent to colling `format!("{}p", si::format(input))`.
///
/// Refer to [`si::format`] to learn the rules that apply.
///
/// # Examples
/// ```
/// use bity::packet::format;
///
/// assert_eq!(format(12), "12p");
/// assert_eq!(format(1_234), "1.23kp");
/// assert_eq!(format(12_000), "12kp");
/// ```
pub fn format(input: u64) -> String {
    format!("{}p", si::format(input))
}

#[cfg(feature = "serde")]
crate::impl_serde!(
    ser:
    /// Serialize a given `u64` into a SI prefixed packet count string.
    ///
    /// Enabling the `serde` allows the use of `#[serde(serialize_with = "bity::packet::serialize")]` and `#[serde(with = "bity::packet")]` attributes.
    ///
    /// ```
    /// use indoc::indoc;
    /// use serde::Serialize;
    ///
    /// #[derive(Serialize)]
    /// #[serde(rename_all = "kebab-case")]
    /// struct Configuration {
    ///     #[serde(serialize_with = "bity::packet::serialize")]
    ///     monthly_usage: u64,
    ///     #[serde(with = "bity::packet")]
    ///     remaining: u64,
    /// }
    ///
    /// assert_eq!(
    ///     toml::to_string(&Configuration {
    ///         monthly_usage: 1_500,
    ///         remaining: 180,
    ///     }).unwrap(),
    ///     indoc! {r#"
    ///         monthly-usage = "1.5kp"
    ///         remaining = "180p"
    ///     "#}
    /// );
    /// ```
    de:
    /// Deserialize a given integer or SI prefixed packet count string into an `u64`.
    ///
    /// Enabling the `serde` allows the use of `#[serde(deserialize_with = "bity::packet::deserialize")]` and `#[serde(with = "bity::packet")]` attributes.
    ///
    /// ```
    /// use indoc::indoc;
    /// use serde::Deserialize;
    ///
    /// #[derive(Deserialize, PartialEq, Debug)]
    /// #[serde(rename_all = "kebab-case")]
    /// struct Configuration {
    ///     #[serde(deserialize_with = "bity::packet::deserialize")]
    ///     monthly_usage: u64,
    ///     #[serde(with = "bity::packet")]
    ///     remaining: u64,
    /// }
    ///
    /// assert_eq!(
    ///     toml::from_str::<Configuration>(
    ///         indoc! {r#"
    ///             monthly-usage = "1.5kp"
    ///             remaining = 180
    ///         "#}
    ///     ).unwrap(),
    ///     Configuration {
    ///         monthly_usage: 1_500,
    ///         remaining: 180,
    ///     }
    /// );
    /// ```
);

#[cfg(test)]
mod tests {
    #[test]
    fn parse() {
        assert_eq!(super::parse("12p").unwrap(), 12);
        assert_eq!(super::parse("12.345kp").unwrap(), 12_345);
        assert_eq!(super::parse("12").unwrap(), 12);
    }

    #[test]
    fn format() {
        assert_eq!(super::format(123), "123p");
        assert_eq!(super::format(1_234), "1.23kp");
        assert_eq!(super::format(12_000), "12kp");
    }
}
