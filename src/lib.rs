use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Sub},
    str::FromStr,
};

pub use error::*;

mod error;
mod tests;

#[cfg(feature = "chrono")]
mod chrono;
#[cfg(feature = "clap")]
mod clap;
#[cfg(feature = "num-traits")]
mod num_traits;
#[cfg(feature = "schemars")]
mod schemars;
#[cfg(feature = "serde")]
mod serde;
#[cfg(feature = "time")]
mod time;

/// 1 kilohertz (kHz) in hertz
pub const KILOHERTZ: u64 = 1_000;

/// 1 megahertz (MHz) in hertz
pub const MEGAHERTZ: u64 = 1_000_000;

/// 1 gigahertz (GHz) in hertz
pub const GIGAHERTZ: u64 = 1_000_000_000;

/// Represents a frequency
///
/// This struct is a wrapper around a `u64` value representing the frequency in hertz.
/// It provides methods to convert between different frequency units (Hz, kHz, MHz, GHz) and
/// to parse frequency strings.
///
/// # Units
/// - Hertz (Hz)
/// - Kilohertz (kHz)
/// - Megahertz (MHz)
/// - Gigahertz (GHz)
///
/// # Note
/// When converting to a string or using display, the frequency is formatted with two decimal places.
/// This is done to provide a consistent representation of the frequency. However, this may lead to
/// precision loss when converting back to a number.
///
/// # Examples
///
/// ```rust
/// use parse_frequency::Frequency;
///
/// let freq = Frequency::from_hz(parse_frequency::GIGAHERTZ);
/// assert_eq!(freq.as_ghz(), 1);
///
/// let freq: Frequency = "2.5GHz".parse().unwrap();
/// assert_eq!(freq.as_hz(), 2_500_000_000);
///
/// let strfreq: String = freq.to_string();
/// assert_eq!(strfreq, "2.50 GHz");
///
/// println!("Frequency: {}", freq);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
#[repr(transparent)]
pub struct Frequency(pub u64);

unsafe impl Send for Frequency {}
unsafe impl Sync for Frequency {}

impl Frequency {
    /// Equivalent to `0 Hz`
    ///
    /// ```rust
    /// # use parse_frequency::Frequency;
    /// assert_eq!(Frequency::ZERO, Frequency::from_hz(0));
    /// ```
    pub const ZERO: Self = Self(0);

    /// Equivalent to `1 Hz`
    ///
    /// ```rust
    /// # use parse_frequency::Frequency;
    /// assert_eq!(Frequency::HERTZ, Frequency::from_hz(1));
    /// ```
    pub const HERTZ: Self = Self(1);

    /// Equivalent to `1 kHz`
    ///
    /// ```rust
    /// # use parse_frequency::Frequency;
    /// assert_eq!(Frequency::KILOHERTZ, Frequency::from_khz(1));
    /// ```
    pub const KILOHERTZ: Self = Self(KILOHERTZ);

    /// Equivalent to `1 MHz`
    ///
    /// ```rust
    /// # use parse_frequency::Frequency;
    /// assert_eq!(Frequency::MEGAHERTZ, Frequency::from_mhz(1));
    /// ```
    pub const MEGAHERTZ: Self = Self(MEGAHERTZ);

    /// Equivalent to `1 GHz`
    ///
    /// ```rust
    /// # use parse_frequency::Frequency;
    /// assert_eq!(Frequency::GIGAHERTZ, Frequency::from_ghz(1));
    /// ```
    pub const GIGAHERTZ: Self = Self(GIGAHERTZ);

    #[must_use]
    pub fn from_hz(hz: u64) -> Self {
        Self(hz)
    }

    #[must_use]
    pub fn from_khz(khz: u64) -> Self {
        Self(khz * KILOHERTZ)
    }

    #[must_use]
    pub fn from_mhz(mhz: u64) -> Self {
        Self(mhz * MEGAHERTZ)
    }

    #[must_use]
    pub fn from_ghz(ghz: u64) -> Self {
        Self(ghz * GIGAHERTZ)
    }

    #[must_use]
    pub fn as_hz(&self) -> u64 {
        self.0
    }

    #[must_use]
    pub fn as_khz(&self) -> u64 {
        self.as_hz() / KILOHERTZ
    }

    #[must_use]
    pub fn as_mhz(&self) -> u64 {
        self.as_hz() / MEGAHERTZ
    }

    #[must_use]
    pub fn as_ghz(&self) -> u64 {
        self.as_hz() / GIGAHERTZ
    }

    /// Converts the frequency to a `std::time::Duration`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parse_frequency::Frequency;
    ///
    /// let freq = Frequency::from_ghz(1);
    /// let duration = freq.as_duration();
    /// assert_eq!(duration.as_nanos(), 1);
    /// ```
    /// # Returns
    /// A `std::time::Duration` representing the frequency.
    #[must_use]
    pub fn as_duration(&self) -> std::time::Duration {
        if self.0 == 0 {
            std::time::Duration::ZERO
        } else {
            std::time::Duration::from_nanos(GIGAHERTZ / self.0)
        }
    }
}

impl Display for Frequency {
    // Precision loss is acceptable here
    #[allow(clippy::cast_precision_loss)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = self.as_hz();

        if value >= GIGAHERTZ {
            write!(f, "{:.2} GHz", value as f64 / GIGAHERTZ as f64)
        } else if value >= MEGAHERTZ {
            write!(f, "{:.2} MHz", value as f64 / MEGAHERTZ as f64)
        } else if value >= KILOHERTZ {
            write!(f, "{:.2} kHz", value as f64 / KILOHERTZ as f64)
        } else {
            write!(f, "{value} Hz")
        }
    }
}

impl FromStr for Frequency {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        parse_frequency(s)
    }
}

impl TryFrom<&str> for Frequency {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self> {
        parse_frequency(s)
    }
}

impl TryFrom<String> for Frequency {
    type Error = Error;

    fn try_from(s: String) -> Result<Self> {
        parse_frequency(&s)
    }
}

impl Add for Frequency {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self(self.0 + other.0)
    }
}

impl Sub for Frequency {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self(self.0 - other.0)
    }
}

impl Mul<u64> for Frequency {
    type Output = Self;

    fn mul(self, rhs: u64) -> Self::Output {
        Self(self.0 * rhs)
    }
}

impl Div<u64> for Frequency {
    type Output = Self;

    fn div(self, rhs: u64) -> Self::Output {
        Self(self.0 / rhs)
    }
}

/// Parses a frequency string and returns a `Frequency` instance.
///
/// # Examples
///
/// ```
/// let freq = parse_frequency::parse_frequency("2.5GHz").unwrap();
/// assert_eq!(freq.as_hz(), 2_500_000_000);
///
/// let freq = parse_frequency::parse_frequency("1.5MHz").unwrap();
/// assert_eq!(freq.as_hz(), 1_500_000);
///
/// let freq = parse_frequency::parse_frequency("500kHz").unwrap();
/// assert_eq!(freq.as_hz(), 500_000);
///
/// let freq = parse_frequency::parse_frequency("100Hz").unwrap();
/// assert_eq!(freq.as_hz(), 100);
///
/// let freq = parse_frequency::parse_frequency("invalid").unwrap_err();
/// assert_eq!(freq.to_string(), "Unknown unit: invalid");
/// ```
///
/// # Errors
///
/// If the input string does not match any of the expected formats (e.g., "1GHz", "2.5MHz", etc.), an error is returned.
pub fn parse_frequency(s: &str) -> Result<Frequency> {
    let s = s.trim().to_lowercase();

    let (value_str, multiplier) = if let Some(value) = s.strip_suffix("ghz") {
        (value, 1_000_000_000)
    } else if let Some(value) = s.strip_suffix("mhz") {
        (value, 1_000_000)
    } else if let Some(value) = s.strip_suffix("khz") {
        (value, 1_000)
    } else if let Some(value) = s.strip_suffix("hz") {
        (value, 1)
    } else {
        return Err(Error::UnknownUnit(s.to_string()));
    };

    let value = value_str
        .trim()
        .parse::<f64>()
        .map_err(|_| Error::InvalidValue(value_str.to_string()))?;

    // Negative values are not allowed
    if value.is_sign_negative() {
        return Err(Error::InvalidValue(value_str.to_string()));
    }

    // It is OK to lose sign and precision here
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let hz = (value * f64::from(multiplier)).round() as u64;
    Ok(Frequency(hz))
}
