use super::{Duration, Time};
use crate::{Error, Result};
use chrono::{Datelike, Months, NaiveDateTime, NaiveTime};
use std::fmt;
use std::ops::Deref;
use std::str::FromStr;

const ONE_MINUTE: &'static str = "1m";
const FIVE_MINUTES: &'static str = "5m";
const TEN_MINUTES: &'static str = "10m";
const ONE_HOUR: &'static str = "1h";
const TWO_HOURS: &'static str = "2h";
const FOUR_HOURS: &'static str = "4h";
const ONE_DAY: &'static str = "1d";
const ONE_WEEK: &'static str = "1w";
const ONE_MONTH: &'static str = "1M";

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum Interval {
    OneMinute,
    FiveMinutes,
    TenMinutes,
    OneHour,
    TwoHours,
    FourHours,
    OneDay,
    OneWeek,
    OneMonth,
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
            Self::OneDay => {
                let duration = Duration::from_days(1);
                time.value() % (duration.value() as i64) == 0
            }
            Self::OneWeek => {
                let offset = Duration::from_days(3); // Thursday through Saturday
                let duration = Duration::from_days(7);
                (time.value() - offset.value() as i64) % (duration.value() as i64) == 0
            }
            Self::OneMonth => {
                let dt = NaiveDateTime::from_timestamp(time.value(), 0);
                dt.date().day() == 1 && dt.time() == NaiveTime::from_hms(0, 0, 0)
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
            Self::OneDay => {
                let duration = Duration::from_days(1);
                time + duration
            }
            Self::OneWeek => {
                let duration = Duration::from_days(7);
                time + duration
            }
            Self::OneMonth => {
                let dt = NaiveDateTime::from_timestamp(time.value(), 0);
                let next_dt = dt.date().checked_add_months(Months::new(1)).unwrap();
                Time::new(next_dt.and_time(dt.time()).timestamp())
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
            Self::OneDay => ONE_DAY,
            Self::OneWeek => ONE_WEEK,
            Self::OneMonth => ONE_MONTH,
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
            ONE_DAY => Ok(Self::OneDay),
            ONE_WEEK => Ok(Self::OneWeek),
            ONE_MONTH => Ok(Self::OneMonth),
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_valid() {
        let tests = vec![
            (
                "2022-10-02T00:00:00",
                [true, true, true, true, true, true, true, true, false],
            ),
            (
                "2023-01-01T00:00:00",
                [true, true, true, true, true, true, true, true, true],
            ),
            (
                "2023-01-01T00:00:10",
                [
                    false, false, false, false, false, false, false, false, false,
                ],
            ),
        ];
        for (s, expected_array) in tests {
            let time = Time::new(NaiveDateTime::from_str(s).unwrap().timestamp());
            assert_eq!(Interval::OneMinute.is_valid_time(&time), expected_array[0]);
            assert_eq!(
                Interval::FiveMinutes.is_valid_time(&time),
                expected_array[1]
            );
            assert_eq!(Interval::TenMinutes.is_valid_time(&time), expected_array[2]);
            assert_eq!(Interval::OneHour.is_valid_time(&time), expected_array[3]);
            assert_eq!(Interval::TwoHours.is_valid_time(&time), expected_array[4]);
            assert_eq!(Interval::FourHours.is_valid_time(&time), expected_array[5]);
            assert_eq!(Interval::OneDay.is_valid_time(&time), expected_array[6]);
            assert_eq!(Interval::OneWeek.is_valid_time(&time), expected_array[7]);
            assert_eq!(Interval::OneMonth.is_valid_time(&time), expected_array[8]);
        }
    }

    #[test]
    fn test_next() {
        let tests = vec![(
            "2023-01-01T00:00:00",
            [
                "2023-01-01T00:01:00",
                "2023-01-01T00:05:00",
                "2023-01-01T00:10:00",
                "2023-01-01T01:00:00",
                "2023-01-01T02:00:00",
                "2023-01-01T04:00:00",
                "2023-01-02T00:00:00",
                "2023-01-08T00:00:00",
                "2023-02-01T00:00:00",
            ],
        )];
        for (s, expected_array) in tests {
            let time = Time::new(NaiveDateTime::from_str(s).unwrap().timestamp());
            let expected_array: Vec<Time> = expected_array
                .into_iter()
                .map(|s| Time::new(NaiveDateTime::from_str(s).unwrap().timestamp()))
                .collect();
            assert_eq!(Interval::OneMinute.next(time), expected_array[0]);
            assert_eq!(Interval::FiveMinutes.next(time), expected_array[1]);
            assert_eq!(Interval::TenMinutes.next(time), expected_array[2]);
            assert_eq!(Interval::OneHour.next(time), expected_array[3]);
            assert_eq!(Interval::TwoHours.next(time), expected_array[4]);
            assert_eq!(Interval::FourHours.next(time), expected_array[5]);
            assert_eq!(Interval::OneDay.next(time), expected_array[6]);
            assert_eq!(Interval::OneWeek.next(time), expected_array[7]);
            assert_eq!(Interval::OneMonth.next(time), expected_array[8]);
        }
    }
}
