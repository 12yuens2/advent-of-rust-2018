extern crate chrono;
extern crate regex;

use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

use regex::Regex;
use chrono::*;

#[derive(Debug)]
enum LogType {
    WakeUp,
    FallAsleep,
    StartShift(String),
}

struct GuardSchedule {
    id: String,
    duty: HashMap<(u32,u32), HashSet<u32>>
}

impl GuardSchedule {
    fn new(id: &str) -> GuardSchedule {
        GuardSchedule {
            id: id.to_string(),
            duty: HashMap::new()
        }
    }
}

fn get_id(line: &str) -> String {
    let segments = line.split(" ").collect::<Vec<&str>>();

    return segments[1].to_string()
}

fn parse_action(action: &str) -> LogType {
    match action {
        "falls asleep" => LogType::FallAsleep,
        "wakes up"     => LogType::WakeUp,
        _              => LogType::StartShift(get_id(action)),
    }
}

fn parse_log(log: &str) -> (DateTime<Utc>, LogType) {
    let pattern = Regex::new(r"\[(.+)\]\s(.+)").unwrap();
    let groups = pattern.captures(log).unwrap();

    let date = Utc.datetime_from_str(&groups[1], "%Y-%m-%d %H:%M").unwrap();
    let action = parse_action(&groups[2]);

    return (date, action);
}

fn get_schedule(events: Vec<(DateTime<Utc>, LogType)>) -> HashMap<String, GuardSchedule> {
    let mut schedules: HashMap<String, GuardSchedule> = HashMap::new();
    let mut current_guard: &str = "1";
    let mut start_time: DateTime<Utc> = Utc::now();

    for (dt, log) in events.iter() {
        let date = (dt.month(), dt.day());

        match log {
            LogType::FallAsleep => {
                let guard: &mut GuardSchedule = schedules.get_mut(current_guard).expect("No guard found");
                guard.duty.entry(date).or_insert(HashSet::new());

                start_time = *dt;
            },

            LogType::WakeUp => {
                let guard: &mut GuardSchedule = schedules.get_mut(current_guard).expect("No guard found");
                let end_time = dt.minute();

                for minute in start_time.minute()..end_time {
                    guard.duty.get_mut(&date).unwrap().insert(minute);
                }
            },

            LogType::StartShift(guard_id) => {
                current_guard = guard_id;

                schedules
                    .entry(current_guard.to_string())
                    .or_insert(GuardSchedule::new(current_guard));
            }
        }
    }

    return schedules;
}


fn most_frequent_minute(guard: &GuardSchedule) -> (u32, u32) {
    let mut minute_map: HashMap<u32, u32> = HashMap::new();
    for minutes in guard.duty.values() {
        for min in minutes {
            let mut count = 0;
            if minute_map.contains_key(min) {
                count = *minute_map.get(min).unwrap();
            }
            minute_map.insert(*min, count + 1);
        }
    }

    let mut count: Vec<(&u32, &u32)> = minute_map.iter().collect();
    count.sort_by(|a, b| b.1.cmp(a.1));

    return (*count[0].0, *count[0].1);
}

fn most_sleeping_guard(schedules: &HashMap<String, GuardSchedule>) -> String {
    let mut sleeping = 0;
    let mut most_sleeping_guard = "1";
    for (guard_id, guard) in schedules.iter() {
        let mut total = 0;
        for ((month, day), minutes) in guard.duty.iter() {
            total += minutes.len();
        }
        if total > sleeping {
            most_sleeping_guard = guard_id;
            sleeping = total;
        }
    }

    return most_sleeping_guard.to_string()
}

fn main() {
    let mut f = File::open("input.data").expect("File not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("Error reading file");

    let lines = contents.split("\n");
    let mut events: Vec<(DateTime<Utc>, LogType)> = lines.map(|line| parse_log(line)).collect();
    events.sort_by_key(|k| k.0);

    let schedules = get_schedule(events);

    //part one
    let guard_id = most_sleeping_guard(&schedules);
    println!("guard {}", guard_id);

    let guard = schedules.get::<str>(&guard_id.to_string()).unwrap();
    let minute = most_frequent_minute(guard).0;
    println!("minute {}", minute);

    //part two
    for (guard_id, guard) in schedules.iter() {
        let freq = most_frequent_minute(guard);
        println!("Guard {}: min {} count {}", guard_id, freq.0, freq.1);
    }
    
    
}
