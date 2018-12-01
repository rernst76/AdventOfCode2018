pub fn frequency_drift(freq_changes : Vec<i32>)  -> i32 {
    freq_changes.iter().sum()
}

#[cfg(test)]
mod tests {
use super::*;

    #[test]
    fn d1_p1_1() {
        let v = vec![1, -2, 3, 1];
        assert_eq!(frequency_drift(v), 3);
    }

    #[test]
    fn d1_p1_2() {
        let v = vec![1, 1, 1];
        assert_eq!(frequency_drift(v), 3);
    }

    #[test]
    fn d1_p1_3() {
        let v = vec![1, 1, -2];
        assert_eq!(frequency_drift(v), 0);
    }

    #[test]
    fn d1_p1_4() {
        let v = vec![-1, -2, -3];
        assert_eq!(frequency_drift(v), -6);
    }
}