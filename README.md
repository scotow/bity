# bity

[SI prefix](https://en.wikipedia.org/wiki/Metric_prefix), data, packets, data-rate, packet-rate string parser and formater.

This crate is mainly targeting network related projects, where configuration
and logs are expressed as bits and packets count.

## Examples

```rust
use indoc::indoc;
use serde::{Deserialize, Serialize};

assert_eq!(bity::si::parse("5.1M").unwrap(), 5_100_000);

assert_eq!(bity::bit::parse("12.34kb").unwrap(), 12_340);
assert_eq!(bity::packet::parse("3.4kp").unwrap(), 3_400);
assert_eq!(bity::bps::parse("8.65kB/s").unwrap(), 69_200);
assert_eq!(bity::pps::parse("2.44Mpps").unwrap(), 2_440_000);

assert_eq!(bity::si::format(5_100_000), "5.1M");
assert_eq!(bity::bit::format(12_340), "12.34kb");
assert_eq!(bity::packet::format(3_400), "3.4kp");
assert_eq!(bity::bps::format(69_200), "69.2kb/s");
assert_eq!(bity::pps::format(2_440_000), "2.44Mp/s");

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "kebab-case")]
struct Configuration {
    #[serde(with = "bity::si")]
    max_users: u64,
    #[serde(with = "bity::bit")]
    user_quota: u64,
    #[serde(with = "bity::bps")]
    bandwidth: u64,
    #[serde(with = "bity::packet")]
    remaining: u64,
    #[serde(with = "bity::pps")]
    record: u64,
}

assert_eq!(
    toml::from_str::<Configuration>(indoc! {r#"
        max-users = "1.5k"
        user-quota = "5.2Gb"
        bandwidth = "512kb/s"
        remaining = "43.88kp"
        record = "88.3Mp/s"
    "#})
    .unwrap(),
    Configuration {
        max_users: 1_500,
        user_quota: 5_200_000_000,
        bandwidth: 512_000,
        remaining: 43_880,
        record: 88_300_000,
    }
);

assert_eq!(
    toml::to_string(&Configuration {
        max_users: 1_500,
        user_quota: 5_200_000_000,
        bandwidth: 512_000,
        remaining: 43_883,
        record: 88_300_000,
    }).unwrap(),
    indoc! {r#"
        max-users = "1.5k"
        user-quota = "5.2Gb"
        bandwidth = "512kb/s"
        remaining = "43.88kp"
        record = "88.3Mp/s"
    "#}
);
```

## Features
- No precision loss
- Differentiate bits and bytes
- `serde` support

## Limitations
- Only support [metric prefixes](https://en.wikipedia.org/wiki/Metric_prefix),
  [IEC prefixes](https://en.wikipedia.org/wiki/Binary_prefix) are not
  supported
- Bit oriented (not byte)
- No customizable formating
- `u64` limited (doesn't go above *exa*, aka. `10^18`)