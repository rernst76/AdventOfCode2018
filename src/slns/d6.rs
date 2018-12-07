use std::cmp::Ordering;

pub fn day_6_solution(input: &str) {

}

fn find_largest_area(nodes: Vec<(i32,i32)>) -> i32 {
    for node in nodes {

    }
    unimplemented!();
}

enum MapArea {
    Some(i32),
    Inf,
    None,
}

fn calculate_area(node: (i32, i32), all_nodes: Vec<(i32,i32)>) -> MapArea {
    // First ensure the node is in the all_nodes set
    unimplemented!();
}

fn get_distance(x: &(i32,i32), y: &(i32, i32)) -> i32 {
    (x.0 - y.0).abs() + (x.1 - y.1).abs()
}

fn get_adjacent(a: (i32,i32)) -> Vec<(i32,i32)> {
    let mut adjacents: Vec<(i32,i32)> = vec![];
    for i in 0..8 {
        let x = a.0;
        let y = a.1;
        match i {
            0 => adjacents.push((x-1, y-1)),
            1 => adjacents.push((x, y-1)),
            2 => adjacents.push((x+1, y-1)),
            3 => adjacents.push((x+1, y)),
            4 => adjacents.push((x+1, y+1)),
            5 => adjacents.push((x, y+1)),
            6 => adjacents.push((x-1, y+1)),
            7 => adjacents.push((x-1, y)),
            _ => panic!(),
        }
    }
    return adjacents;
}

#[derive(PartialEq)]
#[derive(Debug)]
enum ClosestNode {
    Some((i32,i32)),
    Tie,
}

fn get_closest_node(a: (i32, i32), all_nodes: &Vec<(i32, i32)>) -> ClosestNode {
   let mut closest_distance = std::i32::MAX;
   let mut closest_node = ClosestNode::Tie;
   for node in all_nodes {
       let node_distance = get_distance(&a,node);
       match node_distance.cmp(&closest_distance) {
           Ordering::Less    => {
               closest_distance = node_distance;
               closest_node = ClosestNode::Some(*node);
           },
           Ordering::Greater => (),
           Ordering::Equal   => closest_node = ClosestNode::Tie,
       }
   }
   return closest_node;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day_6_example() {
        let points: Vec<(i32,i32)> = vec![(1,1), (1,6), (8,3), (3,4), (5,5), (8,9)];
    }

    #[test]
    fn test_get_distance() {
        let a = (0,0);
        let b = (2,2);
        assert_eq!(get_distance(&a,&b),4);
        let a = (2,3);
        let b = (8,2);
        assert_eq!(get_distance(&a,&b),7);
        let a = (1,1);
        let b = (9,9);
        assert_eq!(get_distance(&a,&b),16);
    }

    #[test]
    fn test_closest_node() {
        let points: Vec<(i32,i32)> = vec![(1,1), (1,6), (8,3), (3,4), (5,5), (8,9)];
        assert_eq!(get_closest_node((1,2), &points) , ClosestNode::Some((1,1)));
        assert_eq!(get_closest_node((9,9), &points) , ClosestNode::Some((8,9)));
        assert_eq!(get_closest_node((4,6), &points) , ClosestNode::Some((5,5)));
        assert_eq!(get_closest_node((0,4), &points) , ClosestNode::Tie);
    }

    #[test]
    fn test_get_adjacent() {
        let answer = vec![(54,4),(55,4),(56,4),(56,5),(56,6),(55,6),(54,6),(54,5)];
        assert_eq!(get_adjacent((55,5)), answer);
    }

}