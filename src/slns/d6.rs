use std::cmp::Ordering;
use std::collections::{HashSet, HashMap, VecDeque};

pub fn day_6_solution(input: &str) {
        let points = input.lines();
        let mut nodes: Vec<(i32,i32)> = vec![];
        for line in points {
            let vals = line.split(',').map(|x| x.trim()).map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>();
            nodes.push((vals[0],vals[1]));
        }
        let area = find_largest_area(nodes);
        println!("Largest node is {:?} with area of {:?}", area.0, area.1);
}


fn find_largest_area(nodes: Vec<(i32,i32)>) -> ((i32, i32), i32) {
    let inf_dist = get_longest_distance(&nodes);
    let mut largest_area_node: ((i32,i32), i32) = ((0,0), 0);

    for node in nodes.iter() {
        if let MapArea::Some(area) = calculate_area(*node, &nodes, inf_dist) {
            if (area > largest_area_node.1) {
                largest_area_node = (*node, area);
            }
        }
    }
    return largest_area_node;
}

fn get_longest_distance(nodes: &Vec<(i32,i32)>) -> i32 {
    let mut greatest_dist = std::i32::MIN;
    for a in nodes {
        for b in nodes {
            let dist = get_distance(&a, &b);
            if dist > greatest_dist {
                greatest_dist = dist;
            }
        }
    }
    return greatest_dist;
}

#[derive(Debug)]
enum MapArea {
    Some(i32),
    Inf,
    None,
}

#[derive(Clone, Copy, Debug)]
struct Point {
    address: (i32,i32),
    address_closest: ClosestNode,
}

fn calculate_area(node: (i32, i32), all_nodes: &Vec<(i32,i32)>, inf_dist: i32) -> MapArea {
    let mut node_queue = VecDeque::new();
    let mut area_count: i32 = 1; // set to one to account for origin node
    let mut adjacent_node_vec = get_adjacent(node);
    let mut known_nodes = HashMap::new();

    // Seed node_queue with adjacent nodes
    adjacent_node_vec.iter().cloned().for_each(|x| {
        if !known_nodes.contains_key(&x) {
            node_queue.push_back(x);
        }
    });

    // Load node list into known nodes
    all_nodes.iter().for_each(|x| {
        known_nodes.insert(*x,Point{address:*x, address_closest:ClosestNode::Some(*x)});
    });

    // Seed current_node before beginning loop
    let mut current_node;
    match node_queue.pop_front() {
        Some(v) => current_node = v,
        None    => current_node = (node.0 + 1, node.1),
    }


    let mut done = false;
    while !done {
        // If we haven't seen this node yet
        if !known_nodes.contains_key(&current_node) {

            // Determine the closest node
            let closest_node = get_closest_node(current_node, &all_nodes);

            // Add this node to known_nodes as we now know what it is closest to
            known_nodes.insert(current_node, Point{address: current_node, address_closest: closest_node});

            if let ClosestNode::Some(v) = closest_node {
                // If node is the closest node, we need to continue processing in this direction
                // add all adjacent nodes to the node_queue, except for ones that are already in the queue
                if v == node {
                    // increment our area_counter as this node contributes to our area!
                    area_count += 1;

                    // Add all adjacent AND unvisited nodes into the queue to be processed
                    adjacent_node_vec = get_adjacent(current_node);
                    adjacent_node_vec.iter().cloned().for_each(|x| {
                        if !node_queue.contains(&x) && !known_nodes.contains_key(&x) {
                            node_queue.push_back(x);
                        }
                    });
                }
            }
        } else { // We have seen the node, check if we need to count it in our area
            match node_queue.pop_front(){
                Some(v) => current_node = v,
                None    => (),
            }
        }
        
        // If we have gone more than half the max distance between nodes, we can assume inf
        if get_distance(&node, &current_node) >= inf_dist {
            return MapArea::Inf;
        }

        // We have run out of nodes, aka found our bounded area
        if node_queue.is_empty() {
            done = true;
        }
    }
    return MapArea::Some(area_count);

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

#[derive(PartialEq, Clone, Copy, Debug)]
enum ClosestNode {
    Some((i32,i32)),
    Tie,
    None,
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
        assert_eq!(find_largest_area(points), 17);
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

    #[test]
    fn test_get_longest_distance() {
        let points: Vec<(i32,i32)> = vec![(1,1), (1,6), (8,3), (3,4), (5,5), (8,9)];
        assert_eq!(get_longest_distance(&points), 15);
    }

}