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
- Optional support for the following features:
  - [`serde`](https://serde.rs/) serialization and deserialization
  - [`clap`](https://docs.rs/clap/) argument parsing
  - [`num-traits`](https://crates.io/crates/num-traits) math functionality
  - [`schemars`](https://crates.io/crates/schemars) JSON documentation support
  - [`time`](https://crates.io/crates/time) duration support
  - [`chrono`](https://crates.io/crates/chrono) duration support

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
parse-frequency = "2.0"
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
parse-frequency provides a number of optional features that enable seamless integration with commonly used Rust libraries. Enable these via Cargo features.

### serde
Enable the `serde` feature to serialize and deserialize `Frequency` as human-readable strings:

```toml
parse-frequency = { version = "...", features = ["serde"] }
```

```rust
use parse_frequency::Frequency;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Config {
    clock: Frequency,
}
```

This allows values like:

```json
{
  "clock": "1.00 GHz"
}
```

### clap
Enable the `clap` feature to use `Frequency` in CLI arguments:

```toml
parse-frequency = { version = "...", features = ["clap"] }
```

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

### enum-traits
Enable `num-traits` to use `Frequency` in generic numeric code (e.g. scientific, DSP, or math contexts):

```toml
parse-frequency = { version = "...", features = ["num-traits"] }
```

Implements:
- `Zero`, `One`
- `Num`, `FromStrRadix`
- `Mul`, `Div`, `Rem`

> Note: Arithmetic is defined in terms of absolute frequency values. Multiplying or dividing two `Frequency` values may not make semantic sense but is supported for compatibility.

### chrono
Enable the `chrono` feature to convert a `Frequency` into a time period:

```toml
parse-frequency = { version = "...", features = ["chrono"] }
```

```rust
use parse_frequency::Frequency;

let freq = Frequency::from_ghz(1);
let period = freq.as_chrono_duration(); // chrono::Duration of 1 ns
```

Handles both low and high frequencies safely using nanosecond/picosecond precision.

### time
Enable the `time` feature to convert a `Frequency` into a `time::Duration`:

```toml
parse-frequency = { version = "...", features = ["time"] }
```

```rust
use parse_frequency::Frequency;

let freq = Frequency::from_mhz(1);
let duration = freq.as_time_duration(); // time::Duration of 1000 ns
```

### schemars
Enable the `schemars` feature to use `Frequency` with OpenAPI / JSON schema generation:

```toml
parse-frequency = { version = "...", features = ["schemars"] }
```

```rust
use parse_frequency::Frequency;
use schemars::JsonSchema;

#[derive(JsonSchema)]
struct Config {
    clock: Frequency,
}
```

Generates a schema like:

```json
{
  "type": "string",
  "format": "frequency",
  "description": "A frequency value like \"2.4 GHz\", \"100 kHz\", or \"440Hz\""
}
```

## Constants

For convenience, the following constants are available:

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