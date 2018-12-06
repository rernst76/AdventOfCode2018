use std::collections::{HashMap, HashSet};

struct Square {
    id: usize,
    x: usize,
    y: usize,
    h: usize,
    w: usize,
}

impl Square {
    fn from_string(s: &str) -> Square {
        // Split on spaces
        let tokens: Vec<&str> = s.split(" ").collect();
        let id = tokens[0]
            .trim_matches('#')
            .parse::<usize>()
            .expect("Failed to parse!");

        // Get position
        let address: Vec<&str> = tokens[2].trim_matches(':').split(',').collect();
        let address: (usize, usize) = (
            address[0].parse::<usize>().unwrap(),
            address[1].parse::<usize>().unwrap(),
        );

        // Get size
        let size: Vec<&str> = tokens[3].split('x').collect();
        let size = (
            size[0].parse::<usize>().unwrap(),
            size[1].parse::<usize>().unwrap(),
        );

        Square {
            id: id,
            x: address.0,
            y: address.1,
            h: size.1,
            w: size.0,
        }
    }
}

pub fn find_num_overlapping_and_best_claim(list: &Vec<&str>) -> (usize, usize) {
    let square_list: Vec<Square> = list.iter().map(|x| Square::from_string(x)).collect();
    let mut square_map: HashMap<(usize, usize), usize> = HashMap::new();

    let mut all_squares = HashSet::new();
    let mut squares = HashMap::new();
    let mut intersecting_squares = HashSet::new();

    for square in square_list {
        let start_x = square.x;
        let start_y = square.y;
        let end_x = start_x + square.w;
        let end_y = start_y + square.h;

        for x in start_x..end_x {
            for y in start_y..end_y {
                *square_map.entry((x, y)).or_insert(0) += 1;

                all_squares.insert(square.id);

                if !squares.contains_key(&(x, y)) {
                    squares.insert((x, y), square.id);
                } else {
                    intersecting_squares.insert(squares[&(x, y)]);
                    intersecting_squares.insert(square.id);
                }
            }
        }
    }

    let answer_1 = square_map.values().filter(|v| **v > 1).count();

    let answer_2 = *all_squares
        .difference(&intersecting_squares)
        .next()
        .unwrap();

    return (answer_1, answer_2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d3_p1_test_square_creation() {
        let test_string = "#1 @ 1,3: 4x4";
        let test_square = Square::from_string(&test_string);

        assert_eq!(test_square.id, 1);
        assert_eq!(test_square.x, 1);
        assert_eq!(test_square.y, 3);
        assert_eq!(test_square.h, 4);
        assert_eq!(test_square.w, 4);
    }

    #[test]
    fn d3_p1_test_square_creation_1() {
        let test_string = "#1218 @ 657,533: 6x3";
        let test_square = Square::from_string(&test_string);

        assert_eq!(test_square.id, 1218);
        assert_eq!(test_square.x, 657);
        assert_eq!(test_square.y, 533);
        assert_eq!(test_square.h, 3);
        assert_eq!(test_square.w, 6);
    }

    #[test]
    fn d3_p1_test_square_creation_2() {
        let test_string = "#123 @ 3,2: 5x4";
        let test_square = Square::from_string(&test_string);

        assert_eq!(test_square.id, 123);
        assert_eq!(test_square.x, 3);
        assert_eq!(test_square.y, 2);
        assert_eq!(test_square.h, 4);
        assert_eq!(test_square.w, 5);
    }
}
