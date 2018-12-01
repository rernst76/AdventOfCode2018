extern crate std;

pub fn frequency_calibrate(freq_changes : Vec<i32>)  -> i32 {
    use std::collections::HashSet;

    // Get a cycle, an endless iterator! Neat!
    let mut it = freq_changes.into_iter().cycle();
    let mut freq = 0;

    let mut freqs = HashSet::new();
    
    loop {
        freq += it.next().expect("Should never happen...");
        if freqs.insert(freq) == false {
            return freq
        }
    }
}

#[cfg(test)]
mod tests {
use super::*;

    #[test]
    fn example_1() {
        let v = vec![1, -2, 3, 1];
        assert_eq!(frequency_calibrate(v), 2);
    }

    #[test]
    fn example_2() {
        let v = vec![1, -1];
        assert_eq!(frequency_calibrate(v), 0);
    }

    #[test]
    fn example_3() {
        let v = vec![3, 3, 4, -2, -4];
        assert_eq!(frequency_calibrate(v), 10);
    }

    #[test]
    fn example_4() {
        let v = vec![-6, 3, 8, 5, -6];
        assert_eq!(frequency_calibrate(v), 5);
    }

    #[test]
    fn example_5() {
        let v = vec![7, 7, -2, -7, -4];
        assert_eq!(frequency_calibrate(v), 14);
    }
}