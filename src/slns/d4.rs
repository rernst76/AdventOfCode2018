extern crate chrono;

use self::chrono::prelude::*;
use std::collections::HashMap;

struct Guard {
    id: usize,
    events:  Vec<SleepEvent>,
}

impl Guard {
    fn add_sleep_event(&mut self, event: SleepEvent) {
        self.events.push(event);
    }
}

struct SleepEvent {
    start: u16,
    end: u16,
}

enum GuardProcedure {
    Begin,
    Sleep,
    Wake,
}

pub fn solve_day_4(entries: &mut Vec<&str>) {
    entries.sort_by(|a,b| {
        let a = str_to_datetime(&a);
        let b = str_to_datetime(&b);
        a.cmp(&b)
    });

    let mut guards = HashMap::new();

    // Generate guard entries
    let mut state = GuardProcedure::Begin;
    let mut guard_id: usize = 0;
    let mut sleep_minute: u16 = 0;
    let mut wake_minute: u16;
    for entry in entries {
        match state {
            GuardProcedure::Begin => {
                state = GuardProcedure::Sleep;
                guard_id = entry.split(|c| c == ' ' || c == '#').nth(4).unwrap().parse().expect("Parse fail.");
            }
            GuardProcedure::Sleep => {
                state = GuardProcedure::Wake;
                sleep_minute = entry.split(|c| c == ' ' || c == ']').nth(1).unwrap().split(':').nth(1).unwrap().parse().expect("Parse fail");
            }
            GuardProcedure::Wake  => {
                state = GuardProcedure::Begin;
                wake_minute = entry.split(|c| c == ' ' || c == ']').nth(1).unwrap().split(':').nth(1).unwrap().parse().expect("Parse fail");

                if !guards.contains_key(&guard_id) {
                    guards.insert(guard_id, Guard {id: guard_id, events: vec![SleepEvent {start: sleep_minute, end: wake_minute}]});
                } else {
                    guards.get_mut(&guard_id).unwrap().add_sleep_event(SleepEvent {start: sleep_minute, end: wake_minute});
                }
            }
        };

        for item in guards.values() {
            println!("{}", item.id);
        }
    }
}

fn str_to_datetime(in_str: &str) -> DateTime<Utc> {
    let date_string = in_str.split(']').next().unwrap().trim_start_matches('[');
    let mut split = date_string.split(' ');
    let ymd_string = split.next().unwrap();
    let ms_string  = split.next().unwrap();

    let mut ymd_split = ymd_string.split('-');
    let year  = ymd_split.next().unwrap().parse::<i32>().unwrap();
    let month = ymd_split.next().unwrap().parse::<u32>().unwrap();
    let day   = ymd_split.next().unwrap().parse::<u32>().unwrap();

    let mut ms_split = ms_string.split(':');
    let hour = ms_split.next().unwrap().parse::<u32>().unwrap();
    let minute = ms_split.next().unwrap().parse::<u32>().unwrap();

    Utc.ymd(year, month, day).and_hms(hour, minute, 0)
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn a_test() {
        assert_eq!(1,1);
    }
}