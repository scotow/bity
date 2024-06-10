use crate::{Error, si};

pub fn parse(input: &str) -> Result<u64, Error> {
    si::parse_with_additional_units(input, &[("p", 1)])
}

pub fn format(input: u64) -> String {
    format!("{}p", si::format(input))
}

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