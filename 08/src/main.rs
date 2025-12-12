//! Day 08: Playground

use std::{
    collections::{BTreeMap, HashMap, HashSet},
    fmt::Debug,
};

/// Struct representing a junction box with its coordinates.
#[derive(Clone, Copy, Hash, Debug, Eq, PartialEq)]
struct JunctionBox {
    x: i64,
    y: i64,
    z: i64,
}

impl JunctionBox {
    /// Make a new JunctionBox from x,y,z coordinates.
    fn new(x: i64, y: i64, z: i64) -> Self {
        Self { x, y, z }
    }

    /// Calculate the distance to other JunctionBox.
    fn distance_to(&self, other: &Self) -> i64 {
        let temp =
            (self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2);
        // I was using this to actually get the "real" distance, but isqrt() rounds.
        // This causes the btree_distances to return 1/4 the number of results.
        // Can't return an f64 since floats aren't hashable...
        // temp.isqrt()
        temp
    }
}

/// Struct representing a circuit of junction boxes.
#[derive(Clone, Eq, PartialEq)]
struct Circuit {
    boxes: HashSet<JunctionBox>,
}

impl Circuit {
    /// Create an initial circuit.
    fn new(jbox: JunctionBox) -> Self {
        Self {
            boxes: HashSet::from([jbox]),
        }
    }

    /// Size of the circuit.
    fn size(&self) -> usize {
        self.boxes.len()
    }

    /// Combine two circuits into one.
    fn combine(&mut self, other: Self) {
        for jbox in other.boxes {
            self.boxes.insert(jbox);
        }
    }
}

impl Debug for Circuit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Circuit:\n")?;
        for jbox in &self.boxes {
            write!(f, "\t{jbox:?}\n")?;
        }
        Ok(())
    }
}

/// Parse the incoming file to VecDeque of JunctionBoxes.
fn parse_text(string: &String) -> Vec<JunctionBox> {
    let mut junction_boxes: Vec<JunctionBox> = Vec::new();

    for line in string.lines() {
        let v: Vec<&str> = line.split(',').collect();
        junction_boxes.push(JunctionBox::new(
            v[0].parse().expect("X Failed to parse."),
            v[1].parse().expect("Y Failed to parse."),
            v[2].parse().expect("Z Failed to parse."),
        ));
    }
    junction_boxes
}

/// Get a BTreeMap of distances with the corresponding JunctionBoxes.
fn btree_of_distances(
    junction_boxes: &Vec<JunctionBox>,
) -> BTreeMap<i64, (JunctionBox, JunctionBox)> {
    let mut ret: BTreeMap<i64, (JunctionBox, JunctionBox)> = BTreeMap::new();

    for a in 0..junction_boxes.len() {
        for b in a + 1..junction_boxes.len() {
            let distance = junction_boxes[a].distance_to(&junction_boxes[b]);
            ret.insert(distance, (junction_boxes[a], junction_boxes[b]));
        }
    }

    ret
}

/// Find the key for where the junction box is in the circuits.
fn find_jbox_circuit(circuits: &HashMap<JunctionBox, Circuit>, jbox: &JunctionBox) -> JunctionBox {
    if circuits.contains_key(jbox) {
        *jbox
    } else {
        *circuits
            .into_iter()
            .find(|x| x.1.boxes.contains(&jbox))
            .expect(&format!("Didn't find jbox in circuits:{:?}", jbox))
            .0
    }
}

/// Take the first quantity of junction_boxes by distance and combine them into
/// circuits.
fn connect_circuits(
    circuits: &mut HashMap<JunctionBox, Circuit>,
    btree_distances: &BTreeMap<i64, (JunctionBox, JunctionBox)>,
    quantity: usize,
) -> i64 {
    // Iterator is already sorted because of BTreeMap.
    for (_, (jbox1, jbox2)) in btree_distances.iter().take(quantity) {
        let jbox1_key = find_jbox_circuit(&circuits, &jbox1);
        let jbox2_key = find_jbox_circuit(&circuits, &jbox2);

        // The two junction boxes are already part of the same circuit. Ignore
        if jbox1_key == jbox2_key {
            continue;
        }

        // For part 2, we catch the last two circuits and product the x's.
        if circuits.len() == 2 {
            return jbox1.x * jbox2.x;
        }

        let circuit2 = circuits
            .remove(&jbox2_key)
            .expect("connect_circuits failed to remove from circuit.");
        circuits
            .entry(jbox1_key)
            .and_modify(|c| c.combine(circuit2));
    }
    0
}

/// Product of the 3 largest circuits. Circuits = number of JunctionBoxes.
fn part1(file_name: &str, number_circuits: usize) -> usize {
    let file_contents = std::fs::read_to_string(file_name).expect("Couldn't open file");
    let junction_boxes = parse_text(&file_contents);
    let btree_distances = btree_of_distances(&junction_boxes);

    let mut circuits = junction_boxes
        .into_iter()
        .map(|jbox| (jbox, Circuit::new(jbox)))
        .collect::<HashMap<JunctionBox, Circuit>>();

    connect_circuits(&mut circuits, &btree_distances, number_circuits);

    let mut sizes: Vec<usize> = circuits.into_iter().map(|c| c.1.size()).collect();
    sizes.sort();

    sizes
        .into_iter()
        .rev()
        .take(3)
        .inspect(|x| println!("{x}"))
        .product()
}

/// Product of the "x" coordinates of the last two junction boxes you need to connect
/// if you connect them all.
fn part2(file_name: &str) -> i64 {
    let file_contents = std::fs::read_to_string(file_name).expect("Couldn't open file");
    let junction_boxes = parse_text(&file_contents);
    let btree_distances = btree_of_distances(&junction_boxes);

    let mut circuits = junction_boxes
        .into_iter()
        .map(|jbox| (jbox, Circuit::new(jbox)))
        .collect::<HashMap<JunctionBox, Circuit>>();

    connect_circuits(&mut circuits, &btree_distances, btree_distances.len())
}

/// Main function / code entry point.
fn main() {
    println!("Sum for example1: {}", part1("example1.txt", 10));
    println!("Sum for input: {}", part1("input.txt", 1000));
    println!("Sum for example1 part2: {}", part2("example1.txt"));
    println!("Sum for input part2: {}", part2("input.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test against the example.
    #[test]
    fn part1_example01() {
        assert_eq!(part1("example1.txt", 10), 40);
    }

    #[test]
    fn test_part1() {
        // assert!(part1("input.txt", 1000) < 260268); // First attempt too high.
        assert_eq!(part1("input.txt", 1000), 175500);
    }

    /// Test against the example.
    #[test]
    fn part2_example01() {
        assert_eq!(part2("example1.txt"), 25272);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("input.txt"), 6934702555);
    }

    // Again, this was working but the rounding from isqrt caused more issues
    // than it was worth getting "real" distances.
    // #[test]
    // fn test_distance_to() {
    //     let j1 = JunctionBox::new(1, 1, 1);
    //     let j2 = JunctionBox::new(2, 1, 1);
    //     let j3 = JunctionBox::new(1, 3, 1);
    //     let j4 = JunctionBox::new(1, 1, 4);
    //     assert_eq!(j1.distance_to(&j2), 1);
    //     assert_eq!(j1.distance_to(&j3), 2);
    //     assert_eq!(j1.distance_to(&j4), 3);
    // }
}
