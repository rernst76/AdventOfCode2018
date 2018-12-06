extern crate chrono;

use self::chrono::prelude::*;
use std::collections::HashMap;

#[derive(Debug)]
struct Guard {
    id: usize,
    events: Vec<(usize, usize)>,
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
    entries.sort_by(|a, b| {
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
                guard_id = entry
                    .split(|c| c == ' ' || c == '#')
                    .nth(4)
                    .unwrap()
                    .parse()
                    .expect("Parse fail.");
            }
            "falls" => {
                sleep_minute = entry
                    .split(|c| c == ' ' || c == ']')
                    .nth(1)
                    .unwrap()
                    .split(':')
                    .nth(1)
                    .unwrap()
                    .parse()
                    .expect("Parse fail");
            }
            "wakes" => {
                wake_minute = entry
                    .split(|c| c == ' ' || c == ']')
                    .nth(1)
                    .unwrap()
                    .split(':')
                    .nth(1)
                    .unwrap()
                    .parse()
                    .expect("Parse fail");

                if !guards.contains_key(&guard_id) {
                    guards.insert(
                        guard_id,
                        Guard {
                            id: guard_id,
                            events: vec![(sleep_minute, wake_minute)],
                        },
                    );
                } else {
                    guards
                        .get_mut(&guard_id)
                        .unwrap()
                        .add_sleep_event((sleep_minute, wake_minute));
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

    println!("Sleepiest Guard is: {}", sleepy_guard.0);
    println!("They slept for: {}", sleepy_guard.1);

    let mut overall_most_overlap: usize = 0;
    let mut overall_best_minute: usize = 0;
    let mut overall_best_guard: Guard = Guard {
        id: 0,
        events: vec![],
    };

    for guard in guards {
        let mut time_accum: [usize; 60] = [0; 60];

        for minutes in &guard.1.events {
            for i in minutes.0..minutes.1 {
                time_accum[i] += 1;
            }
        }
        let winner = time_accum
            .iter()
            .enumerate()
            .map(|(x, y)| (y, x))
            .max()
            .unwrap();
        let minute = winner.1;
        let overlap = winner.0;

        if overlap > &overall_most_overlap {
            overall_most_overlap = *overlap;
            overall_best_minute = minute;
            overall_best_guard = guard.1;
        }
    }
    println!("Overall best minute: {}", overall_best_minute);
    println!("Overall best overlap: {}", overall_most_overlap);
    println!("Overall best guard: {}", overall_best_guard.id);

    //let best_minute = time_accum.iter().enumerate().map(|(x, y)| (y, x)).max().unwrap().1;

    //println!("Best Minute is: {}", best_minute);
}

fn str_to_datetime(in_str: &str) -> DateTime<Utc> {
    let date_string = in_str.split(']').next().unwrap().trim_start_matches('[');
    let mut split = date_string.split(' ');
    let ymd_string = split.next().unwrap();
    let ms_string = split.next().unwrap();

    let mut ymd_split = ymd_string.split('-');
    let year = ymd_split.next().unwrap().parse::<i32>().unwrap();
    let month = ymd_split.next().unwrap().parse::<u32>().unwrap();
    let day = ymd_split.next().unwrap().parse::<u32>().unwrap();

    let mut ms_split = ms_string.split(':');
    let hour = ms_split.next().unwrap().parse::<u32>().unwrap();
    let minute = ms_split.next().unwrap().parse::<u32>().unwrap();

    Utc.ymd(year, month, day).and_hms(hour, minute, 0)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day_4_example() {
        let input = &mut vec![
            "[1518-11-01 00:00] Guard #10 begins shift",
            "[1518-11-01 00:05] falls asleep",
            "[1518-11-01 00:25] wakes up",
            "[1518-11-01 00:30] falls asleep",
            "[1518-11-01 00:55] wakes up",
            "[1518-11-01 23:58] Guard #99 begins shift",
            "[1518-11-02 00:40] falls asleep",
            "[1518-11-02 00:50] wakes up",
            "[1518-11-03 00:05] Guard #10 begins shift",
            "[1518-11-03 00:24] falls asleep",
            "[1518-11-03 00:29] wakes up",
            "[1518-11-04 00:02] Guard #99 begins shift",
            "[1518-11-04 00:36] falls asleep",
            "[1518-11-04 00:46] wakes up",
            "[1518-11-05 00:03] Guard #99 begins shift",
            "[1518-11-05 00:45] falls asleep",
            "[1518-11-05 00:55] wakes up",
        ];
        solve_day_4(input);
        assert_eq!(1, 1);
    }
}
