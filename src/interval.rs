#[derive(PartialEq, PartialOrd, Copy, Clone)]
/// @FIXME We'r reimplementing Rust's Range
pub struct Interval {
    min: f64,
    max: f64,
}

impl Interval {
    pub const fn new(min: f64, max: f64) -> Interval {
        Interval { min, max }
    }

    /// The empty interval, that contains nothing.
    /// ```
    /// use lib::Interval;
    /// let interval = Interval::empty();
    /// assert! (!interval.contains(0.0));
    /// ```
    pub const fn empty() -> Interval {
        Interval::new ( f64::MAX, f64::MIN )
    }

    /// The universe interval, that contains everything.
    /// ```
    /// use lib::Interval;
    /// let interval = Interval::universe();
    /// assert! (interval.contains(0.0));
    /// ```
    pub const fn universe() -> Interval {
        Interval::new ( f64::MIN, f64::MAX )
    }

    /// The positive or null interval, that's 0..f64:MAX
    /// ```
    /// use lib::Interval;
    /// let interval = Interval::positive_or_null();
    /// assert! (interval.contains(0.0));
    /// assert! (!interval.surrounds(0.0));
    /// assert! (!interval.contains(-1.0));
    /// ```
    pub const fn positive_or_null() -> Interval {
        Interval::new(0.0, f64::MAX )
    }

    /// Determine if x is within the `Interval`, inclusve.
    /// ```
    /// use lib::Interval;
    /// let interval = Interval::new(1.0, 2.0);
    /// assert! (!interval.contains(0.5));
    /// assert! (interval.contains(1.0));
    /// assert! (interval.contains(1.5));
    /// assert! (interval.contains(2.0));
    /// assert! (!interval.contains(2.5));
    /// ```
    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    /// Determine if x is within the `Interval`, exclusve.
    /// ```
    /// use lib::Interval;
    /// let interval = Interval::new(1.0, 2.0);
    /// assert! (!interval.surrounds(0.5));
    /// assert! (!interval.surrounds(1.0));
    /// assert! (interval.surrounds(1.5));
    /// assert! (!interval.surrounds(2.0));
    /// assert! (!interval.surrounds(2.5));
    /// ```
    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    /// ```
    /// use lib::Interval;
    /// let interval = Interval::new(1.0, 2.0);
    /// assert! (interval.clamp(0.5)  == 1.0);
    /// assert! (interval.clamp(1.5)  == 1.5);
    /// assert! (interval.clamp(2.5)  == 2.0);
    /// ```
    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min {
            self.min
        } else if x > self.max {
            self.max
        } else {
            x
        }
    }
}

impl Default for Interval {
    fn default() -> Self {
        Interval::new(f64::MIN, f64::MAX)
    }
}
