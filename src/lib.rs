//! [SI prefix](https://en.wikipedia.org/wiki/Metric_prefix), data, packets, data-rate, packet-rate string parser and formater.
//!
//! This crate is mainly targeting network related projects, where configuration
//! and logs are expressed as bits and packets count.
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
//! - Only support [metric prefixes](https://en.wikipedia.org/wiki/Metric_prefix),
//!   [IEC prefixes](https://en.wikipedia.org/wiki/Binary_prefix) are not
//!   supported
//! - Bit oriented (not byte)
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
    // missing_docs
)]
#![deny(unreachable_pub)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![cfg_attr(not(test), warn(clippy::print_stdout, clippy::dbg_macro))]

pub mod bit;
pub mod bps;
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
/// // It will only strip the last per-second instance.
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
