# parse-frequency
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
use parse_frequency::Frequency;

let freq = "2.5GHz".parse::<Frequency>().unwrap();
assert_eq!(freq.to_hz(), 2_500_000_000);
```

### Convert between units
```rust
use parse_frequency::{Frequency, KILOHERTZ};

let f = Frequency::from_khz(2000);
assert_eq!(f.to_mhz(), 2);
assert_eq!(f.to_hz(), 2 * KILOHERTZ * 1000);
```

### Format with display
```rust
let f = Frequency::from_mhz(1337);
println!("{}", f); // → "1337.00 MHz"
```
### Convert to duration
```rust
let f = Frequency::from_ghz(1);
let d = f.as_duration();
assert_eq!(d.as_nanos(), 1);
```