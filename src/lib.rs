use std::fmt::Write;

pub fn parse(input: &str) -> Result<u64, ()> {
    if !input.is_ascii() {
        return Err(());
    }

    let input = input.trim().as_bytes();
    let (value, unit_str) = input.split_at(
        input
            .iter()
            .position(|b| b.is_ascii_alphabetic())
            .unwrap_or(input.len()),
    );
    // SAFETY: The strings are guaranteed to be ascii.
    let (value, unit_str) = unsafe {
        (
            std::str::from_utf8_unchecked(value)
                .trim_matches('0')
                .trim(),
            std::str::from_utf8_unchecked(unit_str),
        )
    };

    if unit_str.len() > 2 {
        return Err(());
    }
    let unit_str_lower = &mut [0; 2][..unit_str.len()];
    unit_str_lower.copy_from_slice(unit_str.as_bytes());
    unit_str_lower.make_ascii_lowercase();
    let unit_str_lower = unit_str_lower.strip_suffix(b"b").unwrap_or(unit_str_lower);

    let (integer_str, fraction_str) = value.split_once('.').unwrap_or((value, ""));
    let mut unit = match &*unit_str_lower {
        b"" => 1,
        b"k" => 1_000,
        b"m" => 1_000_000,
        b"g" => 1_000_000_000,
        b"t" => 1_000_000_000_000,
        b"p" => 1_000_000_000_000_000,
        b"e" => 1_000_000_000_000_000_000,
        _ => return Err(()),
    };
    if unit_str.ends_with("B") {
        unit *= 8;
    }

    fn apply_unit(part: &str, unit: u64, reduce: u64) -> Result<u64, ()> {
        if part.is_empty() {
            return Ok(0);
        }
        Ok(part.parse::<u64>().map_err(|_| ())? * unit / reduce)
    }
    Ok(apply_unit(integer_str, unit, 1)?
        + apply_unit(fraction_str, unit, 10u64.pow(fraction_str.len() as u32))?)
}

pub fn format(input: u64) -> String {
    if input == 0 {
        return "0b".to_owned();
    }

    let exponent = input.ilog10() / 3;
    let unit = match exponent {
        0 => "",
        1 => "k",
        2 => "M",
        3 => "G",
        4 => "T",
        5 => "P",
        6 | _ => "E"
    };

    let mut output = String::with_capacity(8);
    let exponent_base = 10u64.pow(exponent * 3);
    let integer = input / exponent_base;
    write!(output, "{integer}").expect("write error");
    if input % exponent_base != 0 {
        write!(output, ".{0:.2}", (input % exponent_base).to_string().trim_end_matches('0')).expect("write error");
    }
    write!(output, "{unit}b").expect("write error");
    output
}

#[cfg(test)]
mod tests {
    #[test]
    fn parse() {
        assert_eq!(super::parse("12b").unwrap(), 12);
        assert_eq!(super::parse("12B").unwrap(), 96);
        assert_eq!(super::parse("12kb").unwrap(), 12_000);
        assert_eq!(super::parse("12.345kb").unwrap(), 12_345);
        assert_eq!(super::parse("0.12kb").unwrap(), 120);

        assert_eq!(super::parse("12.345kB").unwrap(), 98_760);
        assert_eq!(super::parse("12.3Mb").unwrap(), 12_300_000);
        assert_eq!(super::parse("12.3MB").unwrap(), 98_400_000);
        assert_eq!(super::parse("12.3Gb").unwrap(), 12_300_000_000);
        assert_eq!(super::parse("12.3GB").unwrap(), 98_400_000_000);
        assert_eq!(super::parse("12.3Tb").unwrap(), 12_300_000_000_000);
        assert_eq!(super::parse("12.3TB").unwrap(), 98_400_000_000_000);
        assert_eq!(super::parse("12.3Pb").unwrap(), 12_300_000_000_000_000);
        assert_eq!(super::parse("12.3PB").unwrap(), 98_400_000_000_000_000);

        // "Strange" fractions.
        assert_eq!(super::parse("0.2").unwrap(), 0); // Less than a bit.
        assert_eq!(super::parse("0.125B").unwrap(), 1); // One bit from a byte.
        assert_eq!(super::parse("0.3B").unwrap(), 2); // Round to previous bit.
        assert_eq!(super::parse("12.3B").unwrap(), 98); // Round to previous bit.
        assert_eq!(super::parse("12.34B").unwrap(), 98); // Round to previous bit.
        assert_eq!(super::parse("012.340kb").unwrap(), 12_340); // Unused zeroes.
        assert_eq!(super::parse("12.3456kb").unwrap(), 12_345); // Overflowing fraction.
        assert_eq!(super::parse("12.3456kB").unwrap(), 98_764); // Byte rounding.
        assert_eq!(super::parse("12.34567kB").unwrap(), 98_765); // Byte rounding.
        assert_eq!(super::parse(".5kb").unwrap(), 500); // Missing integer.
        assert_eq!(super::parse("5.kb").unwrap(), 5_000); // Missing fraction.

        // Missing units.
        assert_eq!(super::parse("12k").unwrap(), 12_000);
        assert_eq!(super::parse("12").unwrap(), 12);

        // Additional spaces.
        assert_eq!(super::parse(" 12kb").unwrap(), 12_000);
        assert_eq!(super::parse("12kb ").unwrap(), 12_000);
        assert_eq!(super::parse("12 kb").unwrap(), 12_000);
    }

    #[test]
    fn format() {
        assert_eq!(super::format(0), "0b");
        assert_eq!(super::format(1), "1b");
        assert_eq!(super::format(12), "12b");
        assert_eq!(super::format(123), "123b");
        assert_eq!(super::format(1_234), "1.23kb");
        assert_eq!(super::format(12_000), "12kb");
        assert_eq!(super::format(12_345), "12.34kb");
        assert_eq!(super::format(123_456), "123.45kb");
        assert_eq!(super::format(1_234_567), "1.23Mb");
        assert_eq!(super::format(12_345_678), "12.34Mb");
        assert_eq!(super::format(123_456_789), "123.45Mb");
        assert_eq!(super::format(1_234_567_891), "1.23Gb");
        assert_eq!(super::format(12_345_678_912), "12.34Gb");
        assert_eq!(super::format(12_300_000_000_000), "12.3Tb");
        assert_eq!(super::format(12_300_000_000_000_000), "12.3Pb");
        assert_eq!(super::format(12_300_000_000_000_000_000), "12.3Eb");

        // Extra.
        assert_eq!(super::format(1_200), "1.2kb");
    }
}
