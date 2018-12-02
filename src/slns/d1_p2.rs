extern crate std;

pub fn frequency_calibrate(freq_changes : &Vec<i32>)  -> i32 {
    use std::collections::HashSet;

    // Get a cycle, an endless iterator! Neat!
    let mut it = freq_changes.into_iter().cycle();

    let mut freq = 0;
    let mut freqs = HashSet::new();

    // Insert 0 as we always start at 0
    freqs.insert(0);
    
    loop {
        freq += it.next().expect("Should never happen using a cycle...");
        if freqs.insert(freq) == false {
            return freq
        }
    }
}

#[cfg(test)]
mod tests {
use super::*;

    #[test]
    fn d1_p2_1() {
        let v = vec![1, -2, 3, 1];
        assert_eq!(frequency_calibrate(&v), 2);
    }

    #[test]
    fn d1_p2_2() {
        let v = vec![1, -1];
        assert_eq!(frequency_calibrate(&v), 0);
    }

    #[test]
    fn d1_p2_3() {
        let v = vec![3, 3, 4, -2, -4];
        assert_eq!(frequency_calibrate(&v), 10);
    }

    #[test]
    fn d1_p2_4() {
        let v = vec![-6, 3, 8, 5, -6];
        assert_eq!(frequency_calibrate(&v), 5);
    }

    #[test]
    fn d1_p2_5() {
        let v = vec![7, 7, -2, -7, -4];
        assert_eq!(frequency_calibrate(&v), 14);
    }
}