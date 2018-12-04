extern crate chrono;

use self::chrono::prelude::*;

pub fn solve_day_4(entries: &mut Vec<&str>) {
    entries.sort_by(|a,b| {
        let a = str_to_datetime(&a);
        let b = str_to_datetime(&b);
        a.cmp(&b)
    });

    for val in entries {
        println!("{}", val);
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
    let minute = ms_split.next().unwrap().parse::<u32>().unwrap();
    let second = ms_split.next().unwrap().parse::<u32>().unwrap();

    Utc.ymd(year, month, day).and_hms(0, minute, second)
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn a_test() {
        assert_eq!(1,1);
    }
}