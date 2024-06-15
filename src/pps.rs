use crate::error::Error;
use crate::packet;

pub fn parse(input: &str) -> Result<u64, Error> {
    packet::parse(crate::strip_per_second(input))
}

pub fn format(input: u64) -> String {
    format!("{}/s", packet::format(input))
}

#[cfg(feature = "serde")]
crate::impl_serde!();

#[cfg(test)]
mod tests {
    #[test]
    fn parse() {
        assert_eq!(super::parse("12p/s").unwrap(), 12);
        assert_eq!(super::parse("12pps").unwrap(), 12);
        assert_eq!(super::parse("12.345kp/s").unwrap(), 12_345);
        assert_eq!(super::parse("12.345kpps").unwrap(), 12_345);

        assert_eq!(super::parse("12ps").unwrap(), 12);
        assert_eq!(super::parse("12p").unwrap(), 12);
        assert_eq!(super::parse("12").unwrap(), 12);
    }

    #[test]
    fn format() {
        assert_eq!(super::format(123), "123p/s");
        assert_eq!(super::format(1_234), "1.23kp/s");
        assert_eq!(super::format(12_000), "12kp/s");
    }
}
