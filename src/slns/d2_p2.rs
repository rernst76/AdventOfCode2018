pub fn find_boxes(list: &Vec<&str>) -> Option<(String, String)> {
    for x in list {
        for y in list {
            if hamming_distance(x, y) == 1 {
                return Some((x.to_string(), y.to_string()));
            }
        }
    }
    return None;
}

fn hamming_distance(s1: &str, s2: &str) -> i32 {
    if s1.len() != s2.len() {
        panic!("A mistake has been made!")
    };

    let zip_iter = s1.chars().zip(s2.chars());

    let mut distance = 0;
    for (x, y) in zip_iter {
        if x != y {
            distance += 1;
        }
    }
    return distance;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d2_p1_1() {
        let v = vec![
            "abcde", "fghij", "klmno", "pqrst", "fguij", "axcye", "wvxyz",
        ];
        assert_eq!(
            find_boxes(&v).unwrap(),
            ("fghij".to_string(), "fguij".to_string())
        );
    }

    #[test]
    fn hamming_test_1() {
        let s1 = "abcdef";
        let s2 = "xbcdef";
        assert_eq!(hamming_distance(&s1, &s2), 1);
    }

    #[test]
    fn hamming_test_2() {
        let s1 = "abceef";
        let s2 = "xbcdef";
        assert_eq!(hamming_distance(&s1, &s2), 2);
    }

    #[test]
    fn hamming_test_3() {
        let s1 = "abcdef";
        let s2 = "abcdef";
        assert_eq!(hamming_distance(&s1, &s2), 0);
    }
}
