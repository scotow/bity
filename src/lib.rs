pub mod bit;
pub mod bps;
pub mod pps;
mod si;

fn strip_per_second(mut input: &str) -> &str {
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