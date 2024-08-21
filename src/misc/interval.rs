use std::ops::Add;

/// Represents abstract interval
#[derive(Copy, Clone)]
pub struct Interval {
    /// lower range bound for interval
    pub min: f32,
    /// Upper range bound for interval
    pub max: f32,
}

impl Interval {
    /// Creates new interval
    ///
    /// `min` - lower range bound for interval
    /// `max` - Upper range bound for interval
    pub fn new(min: f32, max: f32) -> Self {
        Self {
            min: if min > max { max } else { min },
            max: if min > max { min } else { max },
        }
    }

    /// Checks if passed `point` is inside given Interval. If not, then returns `false`
    pub fn contains(&self, point: f32) -> bool {
        self.min <= point && point <= self.max
    }

    /// Checks if passed `point` is inside given Interval, excluding borders. If not, then returns `false`
    pub fn surrounds(&self, point: f32) -> bool {
        self.min < point && point < self.max
    }

    /// Returns size of a given Interval
    pub fn size(&self) -> f32 {
        self.max - self.min
    }

    /// Returns new Interval with added padding to both bounds. Used to eliminate pathological intervals
    pub fn add_padding(&self, delta: f32) -> Self {
        let padding = delta / 2.0;
        Self {
            min: self.min - padding,
            max: self.max + padding,
        }
    }

    /// Returns new Interval that should containt each point possible (in float logic of course)
    pub const fn universe() -> Self {
        Self {
            min: f32::MIN,
            max: f32::MAX,
        }
    }
}

impl Add<f32> for Interval {
    type Output = Interval;
    fn add(self, rhs: f32) -> Self {
        Self {
            min: self.min + rhs,
            max: self.max + rhs,
        }
    }
}

impl Default for Interval {
    fn default() -> Self {
        Interval {
            min: f32::MAX,
            max: f32::MIN,
        }
    }
}
