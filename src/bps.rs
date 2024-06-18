//! SI prefixed data-rate parsing and formatting.
//!
//! # Examples
//!
//! ```
//! use bity::bps::{format, parse};
//!
//! assert_eq!(parse("12.3kb/s").unwrap(), 12_300);
//! assert_eq!(parse("0.12kBps").unwrap(), 120 * 8);
//! assert_eq!(parse("12b").unwrap(), 12);
//! assert_eq!(parse("12").unwrap(), 12);
//!
//! assert_eq!(format(1_234), "1.23kb/s");
//! assert_eq!(format(123_456), "123.45kb/s");
//! assert_eq!(format(12_345_678), "12.34Mb/s");
//! ```
//!
//! # Serde
//!
//! Enabling the `serde` allows the use of `#[serde(serialize_with = "bity::bps::serialize")]`,
//! `#[serde(deserialize_with = "bity::bps::deserialize")]` and `#[serde(with = "bity::bps")]`
//! attributes.
//!
//! ```
//! use serde::{Deserialize, Serialize};
//!
//! #[derive(Serialize, Deserialize)]
//! #[serde(rename_all = "kebab-case")]
//! struct Configuration {
//!     name: String,
//!     #[serde(with = "bity::bps")]
//!     bandwidth: u64,
//!     #[serde(with = "bity::bps")]
//!     nic: u64,
//!     #[serde(with = "bity::bps")]
//!     highest: u64,
//! }
//!
//! let config = toml::from_str::<Configuration>(
//!     r#"
//!     name = "module-1"
//!     bandwidth = "5.1Mb/s"
//!     nic = "180kB"
//!     highest = 12_000
//! "#,
//! )
//! .unwrap();
//! assert_eq!(config.bandwidth, 5_100_000);
//! assert_eq!(config.nic, 180 * 1_000 * 8);
//! assert_eq!(config.highest, 12_000);
//!
//! assert_eq!(
//!     toml::to_string(&config).unwrap(),
//!     r#"name = "module-1"
//! bandwidth = "5.1Mb/s"
//! nic = "1.44Mb/s"
//! highest = "12kb/s"
//! "#
//! );
//! ```

use crate::{bit, error::Error};

/// Parse a data-rate SI prefixed string into a number.
///
/// This is equivalent to colling `bit::parse(strip_per_second(input))`.
///
/// Refer to [`si::parse`] and [`bit::parse`] to learn all the rules that apply.
///
/// # Examples
/// ```
/// use bity::bps::parse;
///
/// assert_eq!(parse("12b/s").unwrap(), 12);
/// assert_eq!(parse("12bps").unwrap(), 12);
/// assert_eq!(parse("12.345kb/s").unwrap(), 12_345);
/// assert_eq!(parse("12.345kbps").unwrap(), 12_345);
/// assert_eq!(parse("12b").unwrap(), 12);
/// assert_eq!(parse("12").unwrap(), 12);
/// ```
pub fn parse(input: &str) -> Result<u64, Error<'_>> {
    bit::parse(crate::strip_per_second(input))
}

/// Format an integer into a data-rate SI prefixed string (bit oriented).
///
/// This is equivalent to colling `format!("{}/s", bit::format(input))`.
///
/// Refer to [`si::format`] and [`bit::format`] to learn all the rules that apply.
///
/// # Examples
/// ```
/// use bity::bps::format;
///
/// assert_eq!(format(12), "12b/s");
/// assert_eq!(format(1_234), "1.23kb/s");
/// assert_eq!(format(12_000), "12kb/s");
/// ```
pub fn format(input: u64) -> String {
    format!("{}/s", bit::format(input))
}

#[cfg(feature = "serde")]
crate::impl_serde!(
    ser:
    /// serialize doc
    de:
    /// deserialize doc
);

#[cfg(test)]
mod tests {
    #[test]
    fn parse() {
        assert_eq!(super::parse("12b/s").unwrap(), 12);
        assert_eq!(super::parse("12bps").unwrap(), 12);
        assert_eq!(super::parse("12.345kb/s").unwrap(), 12_345);
        assert_eq!(super::parse("12.345kbps").unwrap(), 12_345);

        assert_eq!(super::parse("12b").unwrap(), 12);
        assert_eq!(super::parse("12").unwrap(), 12);
    }

    #[test]
    fn format() {
        assert_eq!(super::format(123), "123b/s");
        assert_eq!(super::format(1_234), "1.23kb/s");
        assert_eq!(super::format(12_000), "12kb/s");
    }
}
