use std::char;
use std::cmp::Ordering;

#[allow(dead_code)]
pub fn run_reaction(input: &str) {
    println!(
        "No Removed letter Count: {}",
        do_reaction(input).chars().count()
    );
    let alphabet = (10..36)
        .map(|i| char::from_digit(i, 36).unwrap())
        .collect::<Vec<_>>();
    for letter in alphabet {
        let stripped_input = input
            .chars()
            .filter(|x| *x != letter && *x != letter.to_ascii_uppercase())
            .collect::<String>();
        println!(
            "Removed letter: {} | Count: {}",
            letter,
            do_reaction(&stripped_input).chars().count()
        );
    }
}

fn is_reactive(a: char, b: char) -> bool {
    let lower_a = a.to_ascii_lowercase();
    let lower_b = b.to_ascii_lowercase();

    match lower_a.cmp(&lower_b) {
        Ordering::Equal => a.is_ascii_lowercase() != b.is_ascii_lowercase(),
        _ => false,
    }
}

fn find_next_reactive(input: &str) -> Option<(usize, usize)> {
    let sequence = String::from(input);
    let mut iter = sequence.char_indices();
    let mut last_char: (usize, char) = iter.next().unwrap();

    let first_reactive = iter.find(|x| {
        let b = is_reactive(x.1, last_char.1);
        last_char = *x;
        b
    });
    match first_reactive {
        Some(v) => Some((v.0 - 1, v.0)),
        None => None,
    }
}

fn do_reaction(input: &str) -> String {
    let mut sequence = String::from(input);

    loop {
        let next_reactive = find_next_reactive(&sequence);
        match next_reactive {
            Some(v) => {
                sequence.remove(v.0);
                sequence.remove(v.0);
            }
            None => break,
        }
    }
    return sequence;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day_5_example() {
        let input = "dabAcCaCBAcCcaDA";
        assert_eq!(do_reaction(input), "dabCBAcaDA");
    }

    #[test]
    fn test_find_next_reactive() {
        let input = "dabAcCaCBAcCcaDA";
        assert_eq!(find_next_reactive(input), Some((4, 5)));
        let input = "dabAaCcAcCcaDA";
        assert_eq!(find_next_reactive(input), Some((3, 4)));
    }

    #[test]
    fn test_is_reactive() {
        assert_eq!(is_reactive('a', 'a'), false);
        assert_eq!(is_reactive('a', 'b'), false);
        assert_eq!(is_reactive('b', 'a'), false);
        assert_eq!(is_reactive(' ', 'a'), false);
        assert_eq!(is_reactive('A', 'a'), true);
        assert_eq!(is_reactive('A', 'B'), false);
        assert_eq!(is_reactive('B', 'A'), false);
        assert_eq!(is_reactive('b', 'B'), true);
    }
}
