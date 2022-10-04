use std::cmp::Ordering;
use std::ops::Add;

#[derive(Debug, Copy, Clone, Eq)]
pub struct Time {
    timestamp: i64, // in sec
}

impl Time {
    pub fn new(timestamp: i64) -> Self {
        Self { timestamp }
    }

    pub fn value(&self) -> i64 {
        self.timestamp
    }
}

impl PartialEq for Time {
    fn eq(&self, other: &Self) -> bool {
        self.timestamp == other.timestamp
    }
}

impl Ord for Time {
    fn cmp(&self, other: &Self) -> Ordering {
        self.timestamp.cmp(&other.timestamp)
    }
}

impl PartialOrd for Time {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Add<Duration> for Time {
    type Output = Time;
    fn add(self, other: Duration) -> Self::Output {
        Self {
            timestamp: self.timestamp + other.seconds as i64,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq)]
pub struct Duration {
    seconds: u64,
}

impl Duration {
    pub fn value(&self) -> u64 {
        self.seconds
    }
}

impl Duration {
    #[inline]
    pub const fn from_minutes(minutes: u64) -> Self {
        Self {
            seconds: minutes * 60,
        }
    }
    #[inline]
    pub const fn from_hours(hours: u64) -> Self {
        Self::from_minutes(hours * 60)
    }
    #[inline]
    pub const fn from_days(days: u64) -> Self {
        Self::from_hours(days * 24)
    }
}

impl PartialEq for Duration {
    fn eq(&self, other: &Self) -> bool {
        self.seconds == other.seconds
    }
}

impl Ord for Duration {
    fn cmp(&self, other: &Self) -> Ordering {
        self.seconds.cmp(&other.seconds)
    }
}

impl PartialOrd for Duration {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
