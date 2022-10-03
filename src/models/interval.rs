use super::{Duration, Time};
use crate::{Error, Result};
use std::fmt;
use std::ops::Deref;
use std::str::FromStr;

const ONE_MINUTE: &'static str = "1m";
const FIVE_MINUTES: &'static str = "5m";
const TEN_MINUTES: &'static str = "10m";
const ONE_HOUR: &'static str = "1h";
const TWO_HOURS: &'static str = "2h";
const FOUR_HOURS: &'static str = "4h";
// const ONE_DAY: &'static str = "1d";
// const ONE_WEEK: &'static str = "1w";
// const ONE_MONTH: &'static str = "1M";

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum Interval {
    OneMinute,
    FiveMinutes,
    TenMinutes,
    OneHour,
    TwoHours,
    FourHours,
}

impl Interval {
    pub fn as_str(&self) -> &str {
        self.as_ref()
    }

    pub fn is_valid_time(&self, time: &Time) -> bool {
        match self {
            Self::OneMinute => {
                let duration = Duration::from_minutes(1);
                time.value() % (duration.value() as i64) == 0
            }
            Self::FiveMinutes => {
                let duration = Duration::from_minutes(5);
                time.value() % (duration.value() as i64) == 0
            }
            Self::TenMinutes => {
                let duration = Duration::from_minutes(10);
                time.value() % (duration.value() as i64) == 0
            }
            Self::OneHour => {
                let duration = Duration::from_hours(1);
                time.value() % (duration.value() as i64) == 0
            }
            Self::TwoHours => {
                let duration = Duration::from_hours(2);
                time.value() % (duration.value() as i64) == 0
            }
            Self::FourHours => {
                let duration = Duration::from_hours(4);
                time.value() % (duration.value() as i64) == 0
            }
        }
    }

    pub fn next(&self, time: Time) -> Time {
        match self {
            Self::OneMinute => {
                let duration = Duration::from_minutes(1);
                time + duration
            }
            Self::FiveMinutes => {
                let duration = Duration::from_minutes(5);
                time + duration
            }
            Self::TenMinutes => {
                let duration = Duration::from_minutes(10);
                time + duration
            }
            Self::OneHour => {
                let duration = Duration::from_hours(1);
                time + duration
            }
            Self::TwoHours => {
                let duration = Duration::from_hours(2);
                time + duration
            }
            Self::FourHours => {
                let duration = Duration::from_hours(4);
                time + duration
            }
        }
    }
}

impl AsRef<str> for Interval {
    fn as_ref(&self) -> &str {
        match self {
            Self::OneMinute => ONE_MINUTE,
            Self::FiveMinutes => FIVE_MINUTES,
            Self::TenMinutes => TEN_MINUTES,
            Self::OneHour => ONE_HOUR,
            Self::TwoHours => TWO_HOURS,
            Self::FourHours => FOUR_HOURS,
        }
    }
}

impl Deref for Interval {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl Into<String> for Interval {
    fn into(self) -> String {
        self.to_string()
    }
}

impl FromStr for Interval {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self> {
        match value {
            ONE_MINUTE => Ok(Self::OneMinute),
            FIVE_MINUTES => Ok(Self::FiveMinutes),
            TEN_MINUTES => Ok(Self::TenMinutes),
            ONE_HOUR => Ok(Self::OneHour),
            TWO_HOURS => Ok(Self::TwoHours),
            FOUR_HOURS => Ok(Self::FourHours),
            _ => Err(Error::InvalidInterval {
                value: value.to_owned(),
            }),
        }
    }
}

impl From<String> for Interval {
    fn from(value: String) -> Self {
        Self::from_str(&value).unwrap()
    }
}

impl fmt::Display for Interval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
