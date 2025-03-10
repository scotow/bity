//! [SI prefix](https://en.wikipedia.org/wiki/Metric_prefix), data, packets, data-rate, packet-rate string parser and formater.
//!
//! # Examples
//!
//! ```
//! # use indoc::indoc;
//! # use serde::{Deserialize, Serialize};
//! #
//! assert_eq!(bity::si::parse("5.1M").unwrap(), 5_100_000);
//! assert_eq!(bity::bit::parse("12.34kb").unwrap(), 12_340);
//! assert_eq!(bity::bps::parse("8.65kB/s").unwrap(), 69_200);
//! assert_eq!(bity::byte::parse("55.6kB").unwrap(), 55_600);
//! assert_eq!(bity::byteps::parse("94.5kB/s").unwrap(), 94_500);
//! assert_eq!(bity::packet::parse("3.4kp").unwrap(), 3_400);
//! assert_eq!(bity::pps::parse("2.44Mpps").unwrap(), 2_440_000);
//!
//! assert_eq!(bity::si::format(5_100_000), "5.1M");
//! assert_eq!(bity::bit::format(12_340), "12.34kb");
//! assert_eq!(bity::bps::format(69_200), "69.2kb/s");
//! assert_eq!(bity::byte::format(55_600), "55.6kB");
//! assert_eq!(bity::byteps::format(94_500), "94.5kB/s");
//! assert_eq!(bity::packet::format(3_400), "3.4kp");
//! assert_eq!(bity::pps::format(2_440_000), "2.44Mp/s");
//!
//! #[derive(Serialize, Deserialize, PartialEq, Debug)]
//! #[serde(rename_all = "kebab-case")]
//! struct Config {
//!     #[serde(with = "bity::si")]
//!     max_users: u64,
//!     #[serde(with = "bity::bit")]
//!     user_quota: u64,
//!     #[serde(with = "bity::bps")]
//!     bandwidth: u64,
//!     #[serde(with = "bity::byte")]
//!     disk_size: u64,
//!     #[serde(with = "bity::byteps")]
//!     write_speed: u64,
//!     #[serde(with = "bity::packet")]
//!     remaining: u64,
//!     #[serde(with = "bity::pps")]
//!     record: u64,
//! }
//!
//! assert_eq!(
//!     toml::from_str::<Config>(
//!         r#"
//!         max-users = "1.5k"
//!         user-quota = "5.2Gb"
//!         bandwidth = "512kb/s"
//!         disk-size = "88.1TB"
//!         write-speed = "42.9MB/s"
//!         remaining = "43.88kp"
//!         record = "88.3Mp/s"
//!         "#
//!     )
//!     .unwrap(),
//!     Config {
//!         max_users: 1_500,
//!         user_quota: 5_200_000_000,
//!         bandwidth: 512_000,
//!         disk_size: 88_100_000_000_000,
//!         write_speed: 42_900_000,
//!         remaining: 43_880,
//!         record: 88_300_000,
//!     }
//! );
//!
//! assert_eq!(
//!     toml::to_string(&Config {
//!         max_users: 1_500,
//!         user_quota: 5_200_000_000,
//!         bandwidth: 512_000,
//!         disk_size: 88_100_000_000_000,
//!         write_speed: 42_900_000,
//!         remaining: 43_883,
//!         record: 88_300_000,
//!     })
//!     .unwrap(),
//!     indoc! {
//!         r#"
//!         max-users = "1.5k"
//!         user-quota = "5.2Gb"
//!         bandwidth = "512kb/s"
//!         disk-size = "88.1TB"
//!         write-speed = "42.9MB/s"
//!         remaining = "43.88kp"
//!         record = "88.3Mp/s"
//!         "#
//!     }
//! );
//! ```
//!
//! # Features
//! - No precision loss
//! - Differentiate bits and bytes
//! - `serde` support
//!
//! # Limitations
//! - Only support [metric prefixes](https://en.wikipedia.org/wiki/Metric_prefix),
//!   [IEC prefixes](https://en.wikipedia.org/wiki/Binary_prefix) are not
//!   supported
//! - No customizable formating
//! - `u64` limited (doesn't go above *exa*, aka. `10^18`)

#![warn(
    clippy::all,
    clippy::todo,
    clippy::empty_enum,
    clippy::enum_glob_use,
    clippy::mem_forget,
    clippy::unused_self,
    clippy::filter_map_next,
    clippy::needless_continue,
    clippy::needless_borrow,
    clippy::match_wildcard_for_single_variants,
    clippy::if_let_mutex,
    clippy::mismatched_target_os,
    clippy::await_holding_lock,
    clippy::match_on_vec_items,
    clippy::imprecise_flops,
    clippy::suboptimal_flops,
    clippy::lossy_float_literal,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::fn_params_excessive_bools,
    clippy::exit,
    clippy::inefficient_to_string,
    clippy::linkedlist,
    clippy::macro_use_imports,
    clippy::option_option,
    clippy::verbose_file_reads,
    clippy::unnested_or_patterns,
    clippy::str_to_string,
    rust_2018_idioms,
    future_incompatible,
    nonstandard_style,
    missing_debug_implementations,
    missing_docs
)]
#![deny(unreachable_pub)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![cfg_attr(not(test), warn(clippy::print_stdout, clippy::dbg_macro))]

pub mod bit;
pub mod bps;
pub mod byte;
pub mod byteps;
mod error;
pub mod packet;
pub mod pps;
#[cfg(feature = "serde")]
mod serde;
pub mod si;

pub use error::Error;

/// Strip at most one per-second prefix such as `/s` or `ps` (per-second).
///
/// # Examples
///
/// ```
/// assert_eq!(bity::strip_per_second("8kb/s"), "8kb");
/// assert_eq!(bity::strip_per_second("8kbps"), "8kb");
///
/// // It will only strip the last per-second occurrence.
/// assert_eq!(bity::strip_per_second("8kbps/s"), "8kbps");
/// ```
pub fn strip_per_second(mut input: &str) -> &str {
    input = input.trim();
    // Don't use `trim` here because we don't want to remove the suffix multiple
    // times.
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
