use std::fmt::Write;

pub fn parse(input: &str, is_bit: bool) -> Result<u64, ()> {
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
    let unit_str_lower = if is_bit {
        unit_str_lower.strip_suffix(b"b").unwrap_or(unit_str_lower)
    } else {
        unit_str_lower
    };

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
        return "0".to_owned();
    }

    let exponent = input.ilog10() / 3;
    let unit = match exponent {
        0 => "",
        1 => "k",
        2 => "M",
        3 => "G",
        4 => "T",
        5 => "P",
        6 | _ => "E",
    };

    let mut output = String::with_capacity(8);
    let exponent_base = 10u64.pow(exponent * 3);
    let integer = input / exponent_base;
    write!(output, "{integer}").expect("write error");
    if input % exponent_base != 0 {
        write!(
            output,
            ".{0:.2}",
            (input % exponent_base).to_string().trim_end_matches('0')
        )
        .expect("write error");
    }
    write!(output, "{unit}").expect("write error");
    output
}
