use std::cmp::Ordering;

#[allow(dead_code)]
pub fn do_something(input: &str) {
    do_react(input);
}

fn is_reactive(a: char, b: char) -> bool {
    let lower_a = a.to_ascii_lowercase();
    let lower_b = b.to_ascii_lowercase();

    match lower_a.cmp(&lower_b) {
        Ordering::Equal => a.is_ascii_lowercase() != b.is_ascii_lowercase(),
        _               => false,
    }
}

fn do_react(input: &str) {
    let mut sequence = String::from(input);

    loop {
        let remove: Option<(usize,usize)>;

        // Determine if we need to remove anything
        {
            let mut chars = sequence.char_indices();
            let a = chars.next();
            let b = chars.next();

            match (a,b) {
                (Some(_), Some(_)) => (),
                _                  => {
                    break;
                }
            };

            let a: (usize, char) = a.unwrap();
            let b: (usize, char) = b.unwrap();

            match is_reactive(a.1, b.1) {
                true  => {
                    remove = Some((a.0, b.0));
                },
                false => remove = None,
            }
        }
        match remove {
            Some(v) => {
                sequence.remove(v.0);
                sequence.remove(v.1);
            }
            None => ()
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