extern crate chrono;
extern crate itertools;
extern crate regex;
#[macro_use]
extern crate lazy_static;

use chrono::{NaiveDate, NaiveDateTime};
use itertools::Itertools;
use regex::Regex;

const INPUT: &str = include_str!("../input/day04.txt");

fn main() {
    let entries = to_log_entries(INPUT);
    println!("{:?}", entries[0]);
}

fn to_log_entries(input: &str) -> Vec<LogEntry> {
    input
        .lines()
        .map(LogEntry::from_str)
        .sorted_by_key(|entry| entry.date)
}

#[derive(Debug, PartialEq)]
enum GuardAction {
    Sleep,
    Awaken,
    NextGuard(usize), // id of the next guard
}

#[derive(Debug, PartialEq)]
struct LogEntry {
    date: NaiveDateTime,
    action: GuardAction,
}

impl LogEntry {
    fn from_str(s: &str) -> LogEntry {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^\[(\d{4})-(\d{2})-(\d{2}) (\d{2}):(\d{2})\] (\w.*)$").unwrap();
        }

        let captures = RE.captures(s).unwrap();
        let date: NaiveDateTime = NaiveDate::from_ymd(
            captures[1].parse().unwrap(),
            captures[2].parse().unwrap(),
            captures[3].parse().unwrap(),
        ).and_hms(
            captures[4].parse().unwrap(),
            captures[5].parse().unwrap(),
            0,
        );

        let action = GuardAction::from_str(&captures[6]);
        LogEntry { action, date }
    }
}

impl GuardAction {
    fn from_str(s: &str) -> GuardAction {
        lazy_static! {
            static ref ID_RE: Regex = Regex::new(r"\#(\d+)").unwrap();
        }

        let action = if s.starts_with("falls") {
            GuardAction::Sleep
        } else if s.starts_with("wakes") {
            GuardAction::Awaken
        } else {
            let captures = ID_RE.captures(s).unwrap();
            GuardAction::NextGuard(captures[1].parse().unwrap())
        };

        action
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day04_parsing_entry() {
        let test_data = "[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up";

        let log_entries = to_log_entries(test_data);

        assert_eq!(
            log_entries[0],
            LogEntry {
                action: GuardAction::NextGuard(10),
                date: NaiveDate::from_ymd(1518, 11, 1).and_hms(0, 0, 0)
            }
        );

        assert_eq!(
            log_entries[1],
            LogEntry {
                action: GuardAction::Sleep,
                date: NaiveDate::from_ymd(1518, 11, 1).and_hms(0, 5, 0)
            }
        );

        assert_eq!(
            log_entries[2],
            LogEntry {
                action: GuardAction::Awaken,
                date: NaiveDate::from_ymd(1518, 11, 1).and_hms(0, 25, 0)
            }
        );
    }
}
