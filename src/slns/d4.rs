extern crate chrono;

use self::chrono::prelude::*;
use std::collections::HashMap;

struct Guard {
    id: usize,
    events:  Vec<(usize, usize)>,
}

impl Guard {
    fn add_sleep_event(&mut self, event: (usize, usize)) {
        self.events.push(event);
    }

    fn get_total_sleep_time(&self) -> usize {
        self.events.iter().fold(0, |acc, x| acc + (x.1 - x.0))
    }
}

pub fn solve_day_4(entries: &mut Vec<&str>) {
    entries.sort_by(|a,b| {
        let a = str_to_datetime(&a);
        let b = str_to_datetime(&b);
        a.cmp(&b)
    });

    let mut guards = HashMap::new();

    // Generate guard entries
    let mut guard_id: usize = 0;
    let mut sleep_minute: usize = 0;
    let mut wake_minute: usize;
    for entry in entries {
        match entry.split(' ').nth(2).unwrap() {
            "Guard" => {
                guard_id = entry.split(|c| c == ' ' || c == '#').nth(4).unwrap().parse().expect("Parse fail.");
            }
            "falls" => {
                sleep_minute = entry.split(|c| c == ' ' || c == ']').nth(1).unwrap().split(':').nth(1).unwrap().parse().expect("Parse fail");
            }
            "wakes"  => {
                wake_minute = entry.split(|c| c == ' ' || c == ']').nth(1).unwrap().split(':').nth(1).unwrap().parse().expect("Parse fail");

                if !guards.contains_key(&guard_id) {
                    guards.insert(guard_id, Guard {id: guard_id, events: vec![(sleep_minute, wake_minute)]});
                } else {
                    guards.get_mut(&guard_id).unwrap().add_sleep_event((sleep_minute, wake_minute));
                }
            }
            _ => (),
        };
    }

    // Determine guard who sleeps the most
    let sleepy_guard = guards.iter().fold((0usize, 0usize), |acc, x| {
        let sleep_time = x.1.get_total_sleep_time();
        if sleep_time > acc.1 {
            return (*x.0, sleep_time);
        } else {
            return acc;
        }
    });

    println!("{}", sleepy_guard.0);



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