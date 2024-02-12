#[derive(Copy, Clone)]
pub struct Interval {
    pub min: f32,
    pub max: f32,
}

impl Interval {
    pub fn new(min: f32, max: f32) -> Self {
        Self {
            min: if min > max { max } else { min },
            max: if min > max { min } else { max },
        }
    }

    pub fn contains(&self, point: f32) -> bool {
        self.min <= point && point <= self.max
    }

    pub fn surrounds(&self, point: f32) -> bool {
        self.min < point && point < self.max
    }

    pub fn size(&self) -> f32 {
        self.max - self.min
    }

    pub fn add_padding(&self, delta: f32) -> Self {
        let padding = delta / 2.0;
        Self {
            min: self.min - padding,
            max: self.max + padding,
        }
    }

    pub const fn universe() -> Self {
        Self {
            min: f32::MIN,
            max: f32::MAX,
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
