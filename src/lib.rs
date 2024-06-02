pub mod bit;
pub mod bps;
pub mod pps;
mod si;

fn strip_per_second(mut input: &str) -> &str {
    input = input.trim();
    input
        .strip_suffix("/s")
        .or_else(|| input.strip_suffix("ps"))
        .unwrap_or(input)
}