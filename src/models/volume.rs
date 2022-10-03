use super::Quantity;
use std::cmp::Ordering;
use std::ops::AddAssign;

#[derive(Debug, Copy, Clone, Eq)]
pub struct Volume {
    pub num: u64,
    pub scale: u32,
}

impl Volume {
    pub fn new(quantity: Quantity) -> Self {
        Self {
            num: quantity.num,
            scale: quantity.scale,
        }
    }

    pub fn add_quantity(&mut self, quantity: &Quantity) {
        assert_eq!(self.scale, quantity.scale);
        self.num.add_assign(quantity.num);
    }
}

impl PartialEq for Volume {
    fn eq(&self, other: &Self) -> bool {
        self.scale == other.scale && self.num == other.num
    }
}

impl Ord for Volume {
    fn cmp(&self, other: &Self) -> Ordering {
        assert_eq!(self.scale, other.scale);
        self.num.cmp(&other.num)
    }
}

impl PartialOrd for Volume {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
