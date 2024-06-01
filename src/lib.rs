pub fn parse(input: &str) -> Result<u64, ()> {
    let mut parts = input
        .trim()
        .as_bytes()
        .chunk_by(|&b1, &b2| (b1.is_ascii_digit() || b1 == b'.') == (b2.is_ascii_digit() || b2 == b'.'));
    let digits = parts.next().ok_or(())?;
    let unit_str = parts.next().unwrap_or(b"b");
    if parts.next().is_some() {
        return Err(());
    }

    if unit_str.len() > 3 {
        return Err(());
    }
    let mut unit_str_lower = &mut [0; 3][..unit_str.len()];
    unit_str_lower.copy_from_slice(unit_str);
    unit_str_lower.make_ascii_lowercase();
    if matches!(unit_str_lower.last(), Some(b'b')) {
        let len = unit_str_lower.len();
        unit_str_lower = &mut unit_str_lower[..len - 1];
    }

    dbg!(std::str::from_utf8(digits));
    dbg!(std::str::from_utf8(unit_str));
    dbg!(std::str::from_utf8(unit_str_lower));

    let mut digits_parts = digits.split(|&b| b == b'.');
    let decimals = digits_parts.next().ok_or(())?;
    let fractions = digits_parts.next().unwrap_or(b"");
    if digits_parts.next().is_some() {
        return Err(());
    }

    dbg!(std::str::from_utf8(decimals));
    dbg!(std::str::from_utf8(fractions));


    let mut unit = match &*unit_str_lower {
        b"" => 1,
        b"k" => 1_000,
        b"m" => 1_000_000,
        b"g" => 1_000_000_000,
        b"t" => 1_000_000_000_000,
        b"p" => 1_000_000_000_000_000,
        // b"b" => 1,
        // b"B" => 8,
        // b"kb" | b"Kb" => 1_000,
        // b"kB" | b"KB" => 8_000,
        // b"Mb" | b"mb" => 1_000_000,
        // b"MB" | b"mB" => 8_000_000,
        // b"Gb" | b"gb" => 1_000_000_000,
        // b"GB" | b"gB" => 8_000_000_000,
        // b"Tb" | b"tb" => 1_000_000_000_000,
        // b"TB" | b"tB" => 8_000_000_000_000,
        // b"Pb" | b"pb" => 1_000_000_000_000_000,
        // b"PB" | b"pB" => 8_000_000_000_000_000,
        _ => return Err(()),
    };
    if unit_str.ends_with(b"B") {
        unit *= 8;
    }
    dbg!(unit);

    let mut output = 0;
    let mut power = unit;
    for &b in decimals.iter().rev() {
        if !b.is_ascii_digit() {
            return Err(());
        }
        output += (b - b'0') as u64 * power;
        power *= 10;
    }
    // let mut fraction = std::str::from_utf8(fractions).map_err(|_| ())?
    //     .parse::<u64>()?;
    // if unit_str.ends_with(b"B") {
    //     fraction *= 8;
    // }
    // output += fraction /
    power = unit;
    for &b in fractions {
        if !b.is_ascii_digit() {
            return Err(());
        }
        output += (b - b'0') as u64 * power / 10; // Multiply first to keep precision on fractional Byte (B).
        power /= 10;
    }

    Ok(output)
}

#[cfg(test)]
mod tests {
    #[test]
    fn parse() {
        assert_eq!(super::parse("12b").unwrap(), 12);
        assert_eq!(super::parse("12B").unwrap(), 96);
        assert_eq!(super::parse("12kb").unwrap(), 12_000);
        assert_eq!(super::parse("12.345kb").unwrap(), 12_345);

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
        assert_eq!(super::parse("12.3B").unwrap(), 98);
        assert_eq!(super::parse("12.34B").unwrap(), 98);
        assert_eq!(super::parse("12.3456kb").unwrap(), 12_345); // Overflowing fraction.
        assert_eq!(super::parse("12.3456kB").unwrap(), 98_764);
        assert_eq!(super::parse("12.34567kB").unwrap(), 98_765);

        // No units.
        assert_eq!(super::parse("12").unwrap(), 12);
    }
}
