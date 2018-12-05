use std::cmp::Ordering;

#[allow(dead_code)]
pub fn do_something(input: &str) {
    unimplemented!();
}

fn is_reactive(a: char, b: char) -> bool {
    let lower_a = a.to_ascii_lowercase();
    let lower_b = b.to_ascii_lowercase();

    match lower_a.cmp(&lower_b) {
        Ordering::Equal => a.is_ascii_lowercase() != b.is_ascii_lowercase(),
        _               => false,
    }
}

fn do_next_react(input: &str) {
    let mut done = false;

    while !done {
        let mut sequence = String::from(input);
        let mut chars = sequence.char_indices();
        let a = chars.next();
        let b = chars.next();

        match (a,b) {
            (Some(_), Some(_)) => (),
            _                  => {
                done = true;
                break;
            }
        };

        let a: (usize, char) = a.unwrap();
        let b: (usize, char) = b.unwrap();

        match is_reactive(a.1, b.1) {
            true  => {
                sequence.remove(a.0);
                sequence.remove(b.0);
            },
            false => (),
        }
    }
}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day_5_example() {
        assert_eq!(1,1);
    }

    #[test]
    fn day_r_is_reactive_1() {
        assert_eq!(is_reactive('a','a'), false);
        assert_eq!(is_reactive('a','b'), false);
        assert_eq!(is_reactive('b','a'), false);
        assert_eq!(is_reactive(' ','a'), false);
        assert_eq!(is_reactive('A','a'), true);
        assert_eq!(is_reactive('A','B'), false);
        assert_eq!(is_reactive('B','A'), false);
        assert_eq!(is_reactive('b','B'), true);
    }
}