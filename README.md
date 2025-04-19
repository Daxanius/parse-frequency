# parse-frequency
[![crates.io](https://img.shields.io/crates/v/parse-frequency.svg)](https://crates.io/crates/parse-frequency)
[![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](LICENSE-MIT)
[![Rust](https://github.com/Daxanius/parse-frequency/actions/workflows/rust.yml/badge.svg)](https://github.com/Daxanius/parse-frequency/actions/workflows/rust.yml)

A simple, no-nonsense Rust library for parsing frequency strings like `"1.5GHz"` or `"100 kHz"` into strongly-typed frequency values. Case insensitive, of course.

Supports serialization, deserialization, CLI parsing, and conversions between Hz, kHz, MHz, and GHz.

## Features

- Round-trip parsing between strings and frequencies
- Constants for `KILOHERTZ`, `MEGAHERTZ`, and `GIGAHERTZ`
- Optional [serde](https://serde.rs/) support for serialization and deserialization through the serde feature
- Optional [clap](https://docs.rs/clap/) support for CLI arguments through the clap feature
- Built-in formatting (e.g. `"2.50 MHz"`) via `Display`

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
parse-frequency = "1.0"
```

## Usage
### Parse from a string
```rust
let freq = "2.5GHz".parse::<parse_frequency::Frequency>().unwrap();
assert_eq!(freq.to_hz(), 2_500_000_000);
```

### Convert between units
```rust
let f = parse_frequency::Frequency::from_khz(2000);
assert_eq!(f.to_mhz(), 2);
assert_eq!(f.to_hz(), 2 * KILOHERTZ * 1000);
```

### Format with display
```rust
let f = parse_frequency::Frequency::from_mhz(1337);
println!("{}", f); // → "1337.00 MHz"
```
### Convert to duration
```rust
let f = parse_frequency::Frequency::from_ghz(1);
let d = f.as_duration();
assert_eq!(d.as_nanos(), 1);
```