# parse-frequency
[![crates.io](https://img.shields.io/crates/v/parse-frequency.svg)](https://crates.io/crates/parse-frequency)
[![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](LICENSE-MIT)
[![Rust](https://github.com/Daxanius/parse-frequency/actions/workflows/rust.yml/badge.svg)](https://github.com/Daxanius/parse-frequency/actions/workflows/rust.yml)

A simple, no-nonsense Rust library for parsing frequency strings like `"1.5GHz"` or `"100 kHz"` into strongly-typed frequency values. Case insensitive, of course.

Supports serialization, deserialization, CLI parsing, and conversions between Hz, kHz, MHz, and GHz.

## Features

- Parse human-friendly strings like `"1GHz"`, `"2.5 MHz"`, or `"42 kHz"` into a `Frequency` type
- Convert between Hz, kHz, MHz, and GHz with ease
- `Display` implementation (e.g., `"2.50 MHz"`)
- Convert to `std::time::Duration` (period)
- `#[derive(Debug, Copy, Clone, ...)]` with strong type guarantees
- `Send + Sync` support for thread-safe usage in multithreaded environments
- Optional support for:
  - [`serde`](https://serde.rs/) (via the `serde` feature)
  - [`clap`](https://docs.rs/clap/) argument parsing (via the `clap` feature)

## Example

```rust
use parse_frequency::Frequency;

fn main() {
    let cpu_clock = "3.6 GHz".parse::<Frequency>().unwrap();
    println!("CPU clock: {}", cpu_clock);

    let ns_per_cycle = cpu_clock.as_duration();
    println!("One clock cycle takes: {} ns", ns_per_cycle.as_nanos());
}
```

## Installation
Add this to your `Cargo.toml`:

```toml
[dependencies]
parse-frequency = "1.1"
```

Enable optional features:

```toml
[dependencies.parse-frequency]
version = "1.0"
features = ["serde", "clap"]
```

## Quick Start

### Parse from a string

```rust
use parse_frequency::Frequency;

let freq: Frequency = "2.5GHz".parse().unwrap();
assert_eq!(freq.as_hz(), 2_500_000_000);
```

### Convert between units
```rust
use parse_frequency::{Frequency, KILOHERTZ};

let f = Frequency::from_khz(2000);
assert_eq!(f.as_mhz(), 2);
assert_eq!(f.as_hz(), 2 * KILOHERTZ * 1000);
```

### Format for display
Note that due  to the 2 digit precision limitation, the result is rounded when displaying.

```rust
let f = Frequency::from_mhz(1337);
println!("{f}"); // -> "1.34 GHz"
```

### Derive a period as `Duration`
```rust
let f = Frequency::from_ghz(1);
let duration = f.as_duration();

assert_eq!(duration.as_nanos(), 1); // 1 GHz → 1 nanosecond period
```

## Optional Integrations
### Serde
Enable the `serde` feature to serialize and deserialize as strings:

```rust
use parse_frequency::Frequency;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Config {
    clock: Frequency,
}
```

```json
{
  "clock": "1.00 GHz"
}
```

### Clap
Enable the `clap` feature to use `Frequency` directly in CLI argument parsing:

```rust
use clap::Parser;
use parse_frequency::Frequency;

#[derive(Parser)]
struct Args {
    /// Target frequency (e.g. "2.4GHz")
    #[arg(short, long)]
    frequency: Frequency,
}
```

## Constants
These constants are provided for convenience:

```rust
use parse_frequency::{KILOHERTZ, MEGAHERTZ, GIGAHERTZ};

assert_eq!(KILOHERTZ, 1_000);
assert_eq!(MEGAHERTZ, 1_000_000);
assert_eq!(GIGAHERTZ, 1_000_000_000);
```

## Error Handling
The `Frequency::from_str` and `parse_frequency` functions return a custom error enum:

```rust
match "bad input".parse::<Frequency>() {
    Ok(freq) => println!("Parsed: {freq}"),
    Err(e) => eprintln!("Failed: {e}"),
}
```

Example error variants:
- `Error::UnknownUnit("abc")`
- `Error::InvalidValue("GHz")`