//! SI prefix parsing and formatting.
//!
//! # Examples
//!
//! ```
//! use bity::si::{format, parse};
//!
//! assert_eq!(parse("12.3k").unwrap(), 12_300);
//! assert_eq!(parse("0.12k").unwrap(), 120);
//!
//! assert_eq!(format(1_234), "1.23k");
//! assert_eq!(format(123_456), "123.45k");
//! assert_eq!(format(12_345_678), "12.34M");
//! ```
//!
//! # Serde
//!
//! Enabling the `serde` allows the use of `#[serde(serialize_with =
//! "bity::si::serialize")]`, `#[serde(deserialize_with =
//! "bity::si::deserialize")]` and `#[serde(with = "bity::si")]` attributes.
//!
//! ```
//! use indoc::indoc;
//! use serde::{Deserialize, Serialize};
//!
//! #[derive(Serialize, Deserialize, PartialEq, Debug)]
//! #[serde(rename_all = "kebab-case")]
//! struct Configuration {
//!     #[serde(with = "bity::si")]
//!     max_concurrent_users: u64,
//!     #[serde(with = "bity::si")]
//!     instances: u64,
//! }
//!
//! assert_eq!(
//!     toml::from_str::<Configuration>(indoc! {r#"
//!         max-concurrent-users = "1.5k"
//!         instances = 5
//!     "#})
//!     .unwrap(),
//!     Configuration {
//!         max_concurrent_users: 1_500,
//!         instances: 5,
//!     }
//! );
//!
//! assert_eq!(
//!     toml::to_string(&Configuration {
//!         max_concurrent_users: 1_500,
//!         instances: 5,
//!     })
//!     .unwrap(),
//!     indoc! {r#"
//!         max-concurrent-users = "1.5k"
//!         instances = "5"
//!     "#}
//! );
//! ```

use std::fmt::Write;

use crate::error::Error;

const KILO: u64 = 1_000;
const MEGA: u64 = 1_000_000;
const GIGA: u64 = 1_000_000_000;
const TERA: u64 = 1_000_000_000_000;
const PETA: u64 = 1_000_000_000_000_000;
const EXA: u64 = 1_000_000_000_000_000_000;

/// Parse a SI prefixed string into a number.
///
/// Only "positive" and multiple of `1_000^n` prefixes are supported (kilo,
/// mega, ..., upto exa). Whitespaces will be trimmed multiple times at
/// different places, allowing flexible parsing. Because SI prefixes are
/// uniques, the parser in case-insensitive.
///
/// At most one unit must be specified:
/// - `5kk` is not supported for example
/// - if no units is specified, a factor of `1` will be used
///
/// # Examples
/// ```
/// use bity::{si::parse, Error};
///
/// // Basics.
/// assert_eq!(parse("12.3k").unwrap(), 12_300);
/// assert_eq!(parse("0.12k").unwrap(), 120);
/// assert_eq!(parse("12").unwrap(), 12);
/// // "Strange" fractions.
/// assert_eq!(parse("0.2").unwrap(), 0); // Less than a bit.
/// assert_eq!(parse("012.340k").unwrap(), 12_340); // Unused zeroes.
/// assert_eq!(parse("12.3456k").unwrap(), 12_345); // Overflowing fraction.
/// assert_eq!(parse(".5k").unwrap(), 500); // Missing integer.
/// assert_eq!(parse("5.k").unwrap(), 5_000); // Missing fraction.
///                                           // Additional spaces.
/// assert_eq!(parse(" 12k").unwrap(), 12_000);
/// assert_eq!(parse("12k ").unwrap(), 12_000);
/// assert_eq!(parse("12 k").unwrap(), 12_000);
/// // Invalids.
/// assert!(matches!(parse("k"), Err(Error::ParseIntError("", None))));
/// assert!(matches!(parse(".k"), Err(Error::ParseIntError(".", None))));
/// assert!(matches!(parse("1.1."), Err(Error::ParseIntError("1.", Some(_)))));
/// assert!(matches!(parse("1.1.k"), Err(Error::ParseIntError("1.", Some(_)))));
/// assert!(matches!(parse("1.1.1k"), Err(Error::ParseIntError("1.1", Some(_)))));
/// assert!(matches!(parse(".1.1k"), Err(Error::ParseIntError("1.1", Some(_)))));
/// assert!(matches!(parse("12kk"), Err(Error::InvalidUnit("kk"))));
/// assert!(matches!(parse("12kM"), Err(Error::InvalidUnit("kM"))));
/// assert!(matches!(parse("12k M"), Err(Error::InvalidUnit("k M"))));
/// ```
pub fn parse(input: &str) -> Result<u64, Error<'_>> {
    parse_with_additional_units(input, &[])
}

/// Like [`parse`] but with additional units that can be matched after parsing
/// the SI prefixes.
///
/// Like `parse`, at most one additional unit can be used.
///
/// Unlike `parse`, the additional units passed will be matched
/// case-sensitively.
///
/// # Examples
/// ```
/// use bity::si::parse_with_additional_units;
///
/// let additional_units = &[("b", 1), ("B", 8)];
/// assert_eq!(parse_with_additional_units("12", additional_units).unwrap(), 12);
/// assert_eq!(parse_with_additional_units("12b", additional_units).unwrap(), 12 * 1);
/// assert_eq!(parse_with_additional_units("12kB", additional_units).unwrap(), 12 * 1_000 * 8);
/// ```
pub fn parse_with_additional_units<'a>(
    mut input: &'a str,
    additional_units: &[(&str, u64)],
) -> Result<u64, Error<'a>> {
    if !input.is_ascii() {
        return Err(Error::NotAscii);
    }

    input = input.trim();
    let (mut value, original_unit_str) = input.split_at(
        input
            .bytes()
            .position(|b| b.is_ascii_alphabetic())
            .unwrap_or(input.len()),
    );

    let mut unit_str = original_unit_str;
    let mut unit = 1;
    // Look for basic exponent first.
    if !unit_str.is_empty() {
        let exponent = match unit_str.as_bytes()[0].to_ascii_lowercase() {
            b'k' => Some(KILO),
            b'm' => Some(MEGA),
            b'g' => Some(GIGA),
            b't' => Some(TERA),
            b'p' => Some(PETA),
            b'e' => Some(EXA),
            _ => None,
        };
        if let Some(exponent) = exponent {
            if additional_units.iter().all(|(s, _)| *s != &unit_str[..1]) {
                unit *= exponent;
                unit_str = &unit_str[1..];
            }
        }
    }

    // Apply additional unit if one matches.
    if !unit_str.is_empty() {
        for &(additional_unit, addition_factor) in additional_units {
            if unit_str == additional_unit {
                unit *= addition_factor;
                unit_str = "";
                break;
            }
        }
    }

    // Unit parsing should be over by now.
    if !unit_str.is_empty() {
        return Err(Error::InvalidUnit(original_unit_str));
    }

    value = value.trim();
    let (integer_str, mut fraction_str) = value.split_once('.').unwrap_or((value, ""));
    fraction_str = fraction_str.trim_end_matches('0');
    if integer_str.is_empty() && fraction_str.is_empty() {
        return Err(Error::ParseIntError(value, None));
    }

    fn apply_unit(part: &str, unit: u64, reduce: u64) -> Result<u64, Error<'_>> {
        if part.is_empty() {
            return Ok(0);
        }
        Ok(part
            .parse::<u64>()
            .map_err(|err| Error::ParseIntError(part, Some(err)))?
            * unit
            / reduce)
    }
    Ok(apply_unit(integer_str, unit, 1)?
        + apply_unit(fraction_str, unit, 10u64.pow(fraction_str.len() as u32))?)
}

/// Format an integer into a SI prefixed string.
///
/// The first "full" (if any) unit will be used (no `0.**`).
///
/// At most two fraction digits will be displayed.
///
/// # Examples
///
/// ```
/// use bity::si::format;
///
/// assert_eq!(format(0), "0");
/// assert_eq!(format(12), "12");
/// assert_eq!(format(1_234), "1.23k");
/// assert_eq!(format(123_456), "123.45k");
/// assert_eq!(format(12_345_678), "12.34M");
/// assert_eq!(format(1_200_000_000), "1.2G");
/// ```
pub fn format(input: u64) -> String {
    if input == 0 {
        return "0".to_owned();
    }

    let input_str = input.to_string();
    let unit = match (input_str.len() - 1) / 3 {
        0 => "",
        1 => "k",
        2 => "M",
        3 => "G",
        4 => "T",
        5 => "P",
        _ => "E",
    };

    let mut output = String::with_capacity(8);
    let split = (input_str.len() - 1) % 3 + 1;
    write!(output, "{}", &input_str[..split]).expect("write error");
    let fraction_str = input_str[split..].trim_end_matches('0');
    if !fraction_str.is_empty() {
        write!(output, ".{:.2}", fraction_str).expect("write error");
    }
    write!(output, "{unit}").expect("write error");
    output
}

#[cfg(feature = "serde")]
crate::impl_serde!(
    ser:
    /// Serialize a given `u64` into a SI prefixed string.
    ///
    /// Enabling the `serde` allows the use of `#[serde(serialize_with = "bity::si::serialize")]` and `#[serde(with = "bity::si")]` attributes.
    ///
    /// ```
    /// use indoc::indoc;
    /// use serde::Serialize;
    ///
    /// #[derive(Serialize)]
    /// #[serde(rename_all = "kebab-case")]
    /// struct Configuration {
    ///     #[serde(serialize_with = "bity::si::serialize")]
    ///     max_concurrent_users: u64,
    ///     #[serde(with = "bity::si")]
    ///     instances: u64,
    /// }
    ///
    /// assert_eq!(
    ///     toml::to_string(&Configuration {
    ///         max_concurrent_users: 1_500,
    ///         instances: 5,
    ///     }).unwrap(),
    ///     indoc! {r#"
    ///         max-concurrent-users = "1.5k"
    ///         instances = "5"
    ///     "#}
    /// );
    /// ```
    de:
    /// Deserialize a given integer or SI prefixed string into an `u64`.
    ///
    /// Enabling the `serde` allows the use of `#[serde(deserialize_with = "bity::si::deserialize")]` and `#[serde(with = "bity::si")]` attributes.
    ///
    /// ```
    /// use indoc::indoc;
    /// use serde::Deserialize;
    ///
    /// #[derive(Deserialize, PartialEq, Debug)]
    /// #[serde(rename_all = "kebab-case")]
    /// struct Configuration {
    ///     #[serde(deserialize_with = "bity::si::deserialize")]
    ///     max_concurrent_users: u64,
    ///     #[serde(with = "bity::si")]
    ///     instances: u64,
    /// }
    ///
    /// assert_eq!(
    ///     toml::from_str::<Configuration>(
    ///         indoc! {r#"
    ///             max-concurrent-users = "1.5k"
    ///             instances = 5
    ///         "#}
    ///     ).unwrap(),
    ///     Configuration {
    ///         max_concurrent_users: 1_500,
    ///         instances: 5,
    ///     }
    /// );
    /// ```
);

#[cfg(test)]
mod tests {
    use crate::error::Error;

    #[test]
    fn parse() {
        assert_eq!(super::parse("12.345k").unwrap(), 12_345);
        assert_eq!(super::parse("0.12k").unwrap(), 120);

        assert_eq!(super::parse("12").unwrap(), 12);
        assert_eq!(super::parse("12k").unwrap(), 12_000);
        assert_eq!(super::parse("120k").unwrap(), 120_000);
        assert_eq!(super::parse("12.3M").unwrap(), 12_300_000);
        assert_eq!(super::parse("12.3G").unwrap(), 12_300_000_000);
        assert_eq!(super::parse("12.3T").unwrap(), 12_300_000_000_000);
        assert_eq!(super::parse("12.3P").unwrap(), 12_300_000_000_000_000);

        // "Strange" fractions.
        assert_eq!(super::parse("1.02k").unwrap(), 1_020); // Zero at the beginning of the fraction.
        assert_eq!(super::parse("0.2").unwrap(), 0); // Less than one.
        assert_eq!(super::parse("012.340k").unwrap(), 12_340); // Unused zeroes.
        assert_eq!(super::parse("12.3456k").unwrap(), 12_345); // Overflowing fraction.
        assert_eq!(super::parse(".5k").unwrap(), 500); // Missing integer.
        assert_eq!(super::parse("5.k").unwrap(), 5_000); // Missing fraction.

        // Additional spaces.
        assert_eq!(super::parse(" 12k").unwrap(), 12_000);
        assert_eq!(super::parse("12k ").unwrap(), 12_000);
        assert_eq!(super::parse("12 k").unwrap(), 12_000);

        // Invalids.
        assert!(matches!(super::parse("k"), Err(Error::ParseIntError("", None))));
        assert!(matches!(super::parse(".k"), Err(Error::ParseIntError(".", None))));
        assert!(matches!(super::parse("1.1."), Err(Error::ParseIntError("1.", Some(_)))));
        assert!(matches!(super::parse("1.1.k"), Err(Error::ParseIntError("1.", Some(_)))));
        assert!(matches!(super::parse("1.1.1k"), Err(Error::ParseIntError("1.1", Some(_)))));
        assert!(matches!(super::parse(".1.1k"), Err(Error::ParseIntError("1.1", Some(_)))));
        assert!(matches!(super::parse("12kk"), Err(Error::InvalidUnit("kk"))));
        assert!(matches!(super::parse("12kM"), Err(Error::InvalidUnit("kM"))));
        assert!(matches!(super::parse("12k M"), Err(Error::InvalidUnit("k M"))));
    }

    #[test]
    fn parse_with_additional_units() {
        let additional_units = &[("h", 2), ("H", 5)];
        assert_eq!(super::parse_with_additional_units("12", additional_units).unwrap(), 12);
        assert_eq!(super::parse_with_additional_units("12h", additional_units).unwrap(), 12 * 2);
        assert_eq!(super::parse_with_additional_units("12H", additional_units).unwrap(), 12 * 5);
        assert_eq!(
            super::parse_with_additional_units("12.345kh", additional_units).unwrap(),
            12_345 * 2
        );
        assert_eq!(
            super::parse_with_additional_units("12.345kH", additional_units).unwrap(),
            12_345 * 5
        );
        assert_eq!(
            super::parse_with_additional_units("0.12kH", additional_units).unwrap(),
            120 * 5
        );

        assert!(matches!(
            super::parse_with_additional_units("12hh", additional_units),
            Err(Error::InvalidUnit("hh"))
        ));
        assert!(matches!(
            super::parse_with_additional_units("12HH", additional_units),
            Err(Error::InvalidUnit("HH"))
        ));
        assert!(matches!(
            super::parse_with_additional_units("12hH", additional_units),
            Err(Error::InvalidUnit("hH"))
        ));
        assert!(matches!(
            super::parse_with_additional_units("12Hh", additional_units),
            Err(Error::InvalidUnit("Hh"))
        ));
        assert!(matches!(
            super::parse_with_additional_units("12Q", additional_units),
            Err(Error::InvalidUnit("Q"))
        ));

        let additional_units = &[("k", 2)]; // Conflicting units, custom take precedence.
        assert_eq!(super::parse_with_additional_units("12k", additional_units).unwrap(), 24);

        let additional_units = &[("AC", 2)]; // Multi-characters unit.
        assert_eq!(super::parse_with_additional_units("12kAC", additional_units).unwrap(), 24_000);
        assert!(matches!(
            super::parse_with_additional_units("12ACk", additional_units),
            Err(Error::InvalidUnit("ACk"))
        )); // Custom units should come last.
    }

    #[test]
    fn format() {
        assert_eq!(super::format(0), "0");
        assert_eq!(super::format(1), "1");
        assert_eq!(super::format(12), "12");
        assert_eq!(super::format(123), "123");
        assert_eq!(super::format(1_234), "1.23k");
        assert_eq!(super::format(1_023), "1.02k");
        assert_eq!(super::format(12_000), "12k");
        assert_eq!(super::format(12_345), "12.34k");
        assert_eq!(super::format(123_456), "123.45k");
        assert_eq!(super::format(1_234_567), "1.23M");
        assert_eq!(super::format(12_345_678), "12.34M");
        assert_eq!(super::format(123_456_789), "123.45M");
        assert_eq!(super::format(1_234_567_891), "1.23G");
        assert_eq!(super::format(12_345_678_912), "12.34G");
        assert_eq!(super::format(12_300_000_000_000), "12.3T");
        assert_eq!(super::format(12_300_000_000_000_000), "12.3P");
        assert_eq!(super::format(12_300_000_000_000_000_000), "12.3E");

        // Extra.
        assert_eq!(super::format(1_200), "1.2k"); // Zeroes stripped.
    }
}
