//! [SI prefix](https://en.wikipedia.org/wiki/Metric_prefix), data, packets, data-rate, packet-rate string parser and formater.
//!
//! This crate is mainly  ... with network related domain, where configuration and logs are expressed as bits and packets count.
//!
//! # Examples
//!
//! ```
//! assert_eq!(bity::si::parse("5.1M").unwrap(), 5_100_000);
//! assert_eq!(bity::bit::parse("12.34kb").unwrap(), 12_340);
//! assert_eq!(bity::packet::parse("3.4kp").unwrap(), 3_400);
//! assert_eq!(bity::bps::parse("8.65kB/s").unwrap(), 69_200);
//! assert_eq!(bity::pps::parse("2.44Mpps").unwrap(), 2_440_000);
//!
//! assert_eq!(bity::si::format(5_100_000), "5.1M");
//! assert_eq!(bity::bit::format(12_340), "12.34kb");
//! assert_eq!(bity::packet::format(3_400), "3.4kp");
//! assert_eq!(bity::bps::format(69_200), "69.2kb/s");
//! assert_eq!(bity::pps::format(2_440_000), "2.44Mp/s");
//! ```
//!
//! # Features
//! - No precision loss
//! - Differentiate bits and bytes
//!
//! # Limitations
//! - Only support [metric prefixes](https://en.wikipedia.org/wiki/Metric_prefix), [IEC prefixes](https://en.wikipedia.org/wiki/Binary_prefix) are not supported
//! - Bit oriented (not byte)
//! - No customizable formating
//! - `u64` limited (doesn't go above *exa*, aka. `10^18`)

pub mod bit;
pub mod bps;
pub mod pps;
pub mod si;
mod error;
pub mod packet;
#[cfg(feature = "serde")]
mod serde;

pub use error::Error;

/// Strip at most one per-second prefix such as `/s` or `ps` (per-second).
///
/// # Examples
///
/// ```
/// assert_eq!(bity::strip_per_second("8kb/s"), "8kb");
/// assert_eq!(bity::strip_per_second("8kbps"), "8kb");
///
/// // It will only strip the last per-second instance.
/// assert_eq!(bity::strip_per_second("8kbps/s"), "8kbps");
/// ```
pub fn strip_per_second(mut input: &str) -> &str {
    input = input.trim();
    // Don't use `trim` here because we don't want to remove the suffix multiple times.
    input
        .strip_suffix("/s")
        .or_else(|| input.strip_suffix("ps"))
        .unwrap_or(input)
}

#[cfg(test)]
mod tests {
    #[test]
    fn strip_per_second() {
        assert_eq!(super::strip_per_second("whatever/s"), "whatever");
        assert_eq!(super::strip_per_second("whateverps"), "whatever");
        assert_eq!(super::strip_per_second("whatever/s/s"), "whatever/s");
        assert_eq!(super::strip_per_second("whateverpsps"), "whateverps");
        assert_eq!(super::strip_per_second("whateverps/s"), "whateverps");
        assert_eq!(super::strip_per_second("whatever/sps"), "whatever/s");
    }
}