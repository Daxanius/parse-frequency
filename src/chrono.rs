use super::Frequency;
use chrono::Duration;

impl Frequency {
    /// Converts the frequency to a `chrono::Duration`.
    ///
    /// # Examples
    /// ```rust
    /// use parse_frequency::Frequency;
    ///
    /// let freq = Frequency::from_ghz(1);
    /// let duration = freq.as_chrono_duration();
    /// assert_eq!(duration.num_nanoseconds(), Some(1));
    ///
    /// let freq = Frequency::from_mhz(1);
    /// let duration = freq.as_chrono_duration();
    /// assert_eq!(duration.num_nanoseconds(), Some(1_000));
    /// ````
    #[must_use]
    #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
    pub fn as_chrono_duration(&self) -> chrono::Duration {
        if self.0 == 0 {
            return Duration::zero();
        }

        let nanoseconds_per_second: u64 = 1_000_000_000;

        if nanoseconds_per_second >= self.0 {
            Duration::nanoseconds((nanoseconds_per_second / self.0) as i64)
        } else {
            // If frequency is higher than 1 GHz, the period is less than 1 ns.
            // Calculate in picoseconds and then convert to nanoseconds.
            let picoseconds_per_second: u128 = 1_000_000_000_000;
            let frequency: u128 = u128::from(self.0);
            let period_in_picoseconds = picoseconds_per_second / frequency;
            let period_in_nanoseconds = period_in_picoseconds / 1_000;
            Duration::nanoseconds(period_in_nanoseconds as i64)
        }
    }
}
