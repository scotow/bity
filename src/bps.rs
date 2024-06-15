use crate::bit;
use crate::error::Error;

pub fn parse(input: &str) -> Result<u64, Error> {
    bit::parse(crate::strip_per_second(input))
}

pub fn format(input: u64) -> String {
    format!("{}/s", bit::format(input))
}

#[cfg(feature = "serde")]
crate::impl_serde!();

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
