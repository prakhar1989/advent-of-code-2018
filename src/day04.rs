extern crate chrono;
extern crate itertools;
extern crate regex;
#[macro_use]
extern crate lazy_static;

use chrono::{NaiveDate, NaiveDateTime, Timelike};
use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;

const INPUT: &str = include_str!("../input/day04.txt");

fn main() {
    let entries = to_log_entries(INPUT);
    println!("Part 1: {}", part1(&entries));
    println!("Part 2: {}", part2(&entries));
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

fn build_sleeping_pattern(entries: &Vec<LogEntry>) -> HashMap<usize, [usize; 60]> {
    let mut sleep_map = HashMap::new();
    let mut current_guard_id = 0;
    let mut current_sleeping_minute = 0;
    for entry in entries {
        match entry.action {
            GuardAction::Sleep => {
                current_sleeping_minute = entry.date.minute();
            }
            GuardAction::NextGuard(id) => {
                current_guard_id = id;
            }
            GuardAction::Awaken => {
                let sleep_history = sleep_map.entry(current_guard_id).or_insert([0; 60]);
                for minute in current_sleeping_minute..(entry.date.minute()) {
                    sleep_history[minute as usize] += 1;
                }
            }
        }
    }
    sleep_map
}

fn part1(entries: &Vec<LogEntry>) -> usize {
    let sleep_map = build_sleeping_pattern(entries);

    let most_sleepy_guard = sleep_map
        .iter()
        .map(|(id, history)| (id, history.iter().sum::<usize>()))
        .max_by_key(|(_, total)| *total)
        .unwrap()
        .0;

    *most_sleepy_guard * most_sleepy_minute(&sleep_map[most_sleepy_guard])
}

fn part2(entries: &Vec<LogEntry>) -> usize {
    let sleep_map = build_sleeping_pattern(entries);

    let (guard_id, most_sleepy_minute, _) = sleep_map
        .iter()
        .map(|(id, history)| {
            let m = most_sleepy_minute(history);
            (
                id, m,          /* most sleepy minute */
                history[m], /* times slept */
            )
        }).max_by_key(|(_, _, times_slept)| *times_slept)
        .unwrap();

    guard_id * most_sleepy_minute
}

fn most_sleepy_minute(history: &[usize; 60]) -> usize {
    history
        .iter()
        .enumerate()
        .max_by_key(|(_, total)| *total)
        .unwrap()
        .0
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
        let log_entries = to_log_entries(&test_data());

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

    #[test]
    fn day04_build_sleeping_pattern() {
        let log_entries = to_log_entries(&test_data());
        let sleep_map = build_sleeping_pattern(&log_entries);

        assert_eq!(sleep_map.contains_key(&10), true);
        assert_eq!(sleep_map.contains_key(&99), true);
        assert_eq!(sleep_map.contains_key(&999), false);

        let minute_history1 = sleep_map.get(&10).unwrap();
        assert_eq!(minute_history1[24], 2);
        assert_eq!(minute_history1.iter().sum::<usize>(), 50);

        let minute_history2 = sleep_map.get(&99).unwrap();
        assert_eq!(minute_history2[45], 3);
        assert_eq!(minute_history2.iter().sum::<usize>(), 30);
    }

    #[test]
    fn day04_part1() {
        let log_entries = to_log_entries(&test_data());
        assert_eq!(part1(&log_entries), 240);
    }

    #[test]
    fn day04_part2() {
        let log_entries = to_log_entries(&test_data());
        assert_eq!(part2(&log_entries), 4455);
    }

    fn test_data() -> String {
        "[1518-11-01 00:00] Guard #10 begins shift
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
[1518-11-05 00:55] wakes up"
            .to_string()
    }
}
