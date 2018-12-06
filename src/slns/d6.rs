pub fn day_6_solution(input: &str) {

}

fn find_largest_area(nodes: Vec<(usize,usize)>) -> usize {
    for node in nodes {

    }
}

enum MapArea {
    Some(usize),
    Inf,
    None,
}

fn calculate_area(node: (usize, usize), all_nodes: Vec<(usize,usize)>) -> MapArea {
    // First ensure the node is in the all_nodes set
    match all_nodes.iter().find(|x| x==node) {
        Some(v) => (),
        None    => return MapArea::None,
    };


}

fn get_distance(a: (usize,usize), b: (usize, usize)) -> usize {
    ((a.0 - b.0).abs()) + ((a.1 - b.1).abs())
}

fn get_adjacent(a: (usize,usize)) -> [(usize, usize); 8] {
    [(a-1,a-1), (a, a-1), (a+1, a-1), (a+1, a), (a+1, a+1), (a, a+1), (a-1, a+1), (a-1, a)]
}

#[cfg(test)]
mod test {
    use super::*;

    fn day_6_example {
        let points: Vec<(usize,usize)> = vec![(1,1), (1,6), (8,3), (3,4), (5,5), (8,9)];
    }

}