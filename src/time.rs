use crate::Frequency;

impl Frequency {
    /// Converts the frequency to a `time::Duration`.
    ///
    /// # Examples
    /// ```rust
    /// use parse_frequency::Frequency;
    ///
    /// let freq = Frequency::from_ghz(1);
    /// let duration = freq.as_time_duration();
    /// assert_eq!(duration.whole_nanoseconds(), 1);
    /// ```
    #[must_use]
    #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
    pub fn as_time_duration(&self) -> time::Duration {
        if self.0 == 0 {
            return time::Duration::ZERO;
        }

        let nanoseconds_per_second: u64 = crate::GIGAHERTZ;

        // Calculate the period in nanoseconds.
        // To avoid potential overflow if self.0 is small, we perform the division last.
        if nanoseconds_per_second >= self.0 {
            time::Duration::nanoseconds((nanoseconds_per_second / self.0) as i64)
        } else {
            // If frequency is higher than 1 GHz, the period is less than 1 ns.
            // We need to handle this carefully. We can calculate the reciprocal
            // as a fraction and then convert to nanoseconds, potentially losing
            // some precision for extremely high frequencies.
            let picoseconds_per_second: u128 = 1_000_000_000_000;
            let frequency: u128 = u128::from(self.0);
            let period_in_picoseconds = picoseconds_per_second / frequency;

            // Convert picoseconds to nanoseconds (integer division will truncate)
            let period_in_nanoseconds = period_in_picoseconds / 1_000;
            time::Duration::nanoseconds(period_in_nanoseconds as i64)
        }
    }
}
