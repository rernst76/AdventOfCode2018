pub fn calculate_checksum(list: &Vec<&str>) -> i32 {
    use std::collections::HashMap;

    let mut three_count = 0;
    let mut two_count = 0;

    let mut counts = HashMap::new();

    for val in list {
        for c in val.chars() {
            let count = match counts.get(&c) {
                Some(v) => v + 1,
                None => 1,
            };
            counts.insert(c, count);
        }
        if counts.values().any(|&x| x == 3) {
            three_count += 1;
        }
        if counts.values().any(|&x| x == 2) {
            two_count += 1;
        }
        counts.clear();
    }
    three_count * two_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d2_p1_1() {
        let v = vec![
            "abcdef", "bababc", "abbcde", "abcccd", "aabcdd", "abcdee", "ababab",
        ];
        assert_eq!(calculate_checksum(&v), 12);
    }
}
