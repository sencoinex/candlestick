use std::cmp::Ordering;

#[derive(Debug, Copy, Clone, Eq)]
pub struct Quantity {
    pub num: u64,
    pub scale: u32,
}

impl PartialEq for Quantity {
    fn eq(&self, other: &Self) -> bool {
        self.scale == other.scale && self.num == other.num
    }
}

impl Ord for Quantity {
    fn cmp(&self, other: &Self) -> Ordering {
        assert_eq!(self.scale, other.scale);
        self.num.cmp(&other.num)
    }
}

impl PartialOrd for Quantity {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
