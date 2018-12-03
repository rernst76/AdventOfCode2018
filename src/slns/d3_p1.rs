struct Square {
    id: i32,
    x: i32,
    y: i32,
    h: i32,
    w: i32,
}

impl Square {
    fn from_string(s: &str) -> Square {
        // Split on spaces
        let tokens: Vec<&str> = s.split(" ").collect();
        let id = tokens[0].trim_matches('#').parse::<i32>().expect("Failed to parse!");

        // Get position
        let address: Vec<&str> = tokens[2].trim_matches(':').split(',').collect();
        let address: (i32, i32) = (address[0].parse::<i32>().unwrap(), address[1].parse::<i32>().unwrap());

        // Get size
        let size: Vec<&str> = tokens[3].split('x').collect();
        let size = (size[0].parse::<i32>().unwrap(), size[1].parse::<i32>().unwrap());

        Square {
            id: id,
            x: address.0,
            y: address.1,
            h: size.0,
            w: size.1,
        }
    }
}

pub fn find_num_overlapping(list: &Vec<&str>) -> i32 {
    let square_list: Vec<Square> = list.iter().map(|x| Square::from_string(x)).collect();
    let mut square_map: [[i32; 1000]; 1000] = [[0; 1000]; 1000];
    for square in square_list {
        insert_square(&mut square_map, &square);
    }

    let mut sum_overlapped_squares = 0;

    for x in 0..999 {
        for y in 0..999 {
            let ix = x as usize;
            let iy = y as usize;
            if square_map[ix][iy] > 1 {
                sum_overlapped_squares += 1;
            }
        }
    }

    return sum_overlapped_squares;
}

fn insert_square(square_map: &mut [[i32; 1000]; 1000], square: &Square) {
    for x in square.x .. (square.x + square.w) {
        for y in square.y .. (square.y + square.h) {
            let ix = x as usize;
            let iy = y as usize;
            square_map[ix][iy] = square_map[ix][iy] + 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d3_p1_1() {
        let v = vec![
            "#1 @ 1,3: 4x4", "#2 @ 3,1: 4x4", "#3 @ 5,5: 2x2"
        ];
        assert_eq!(find_num_overlapping(&v), 4);
    }

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
        assert_eq!(test_square.h, 6);
        assert_eq!(test_square.w, 3);
    }
}