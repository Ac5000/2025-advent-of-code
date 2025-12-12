//! Day 09: Movie Theater

use utilities::coord::Coord;

/// Parse the incoming file to Vec of Coords.
fn parse_text(string: &String) -> Vec<Coord> {
    let mut coords: Vec<Coord> = Vec::new();

    for line in string.lines() {
        let v: Vec<&str> = line.split(',').collect();
        coords.push(Coord::new(
            v[0].parse().expect("X Failed to parse."),
            v[1].parse().expect("Y Failed to parse."),
        ));
    }
    coords
}

/// Find the biggest area between Coords
fn get_biggest_area(coords: &Vec<Coord>) -> (Coord, Coord, u64) {
    let mut biggest_area = u64::MIN;
    let mut coord1: Coord = Coord::new(-1, -1);
    let mut coord2: Coord = Coord::new(-1, -1);

    for a in 0..coords.len() {
        for b in a + 1..coords.len() {
            let area = calculate_area(&coords[a], &coords[b]);
            if area > biggest_area {
                biggest_area = area;
                coord1 = coords[a];
                coord2 = coords[b];
            }
        }
    }

    (coord1, coord2, biggest_area)
}

/// Calculate the area of the rectangle between these two points.
fn calculate_area(coord1: &Coord, coord2: &Coord) -> u64 {
    // +1s necessary since area includes the coords locations.
    let x_axis = u64::from(coord1.x.abs_diff(coord2.x) + 1);
    let y_axis = u64::from(coord1.y.abs_diff(coord2.y) + 1);
    x_axis * y_axis
}

/// Product of the 3 largest circuits. Circuits = number of JunctionBoxes.
fn part1(file_name: &str) -> u64 {
    let file_contents = std::fs::read_to_string(file_name).expect("Couldn't open file");
    let tiles = parse_text(&file_contents);
    let (_coord1, _coord2, area) = get_biggest_area(&tiles);
    // println!("{_coord1}, {_coord2}");
    area
}

/// Main function / code entry point.
fn main() {
    println!("Sum for example1: {}", part1("example1.txt"));
    println!("Sum for input: {}", part1("input.txt"));
    // println!("Sum for example1 part2: {}", part2("example1.txt"));
    // println!("Sum for input part2: {}", part2("input.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test against the example.
    #[test]
    fn part1_example01() {
        assert_eq!(part1("example1.txt"), 50);
    }

    #[test]
    fn test_part1() {
        // 4225115601 too low
        let part1 = part1("input.txt");
        assert!(part1 > 4225115601); // First attempt too low.
        assert_eq!(part1, 4759420470);
    }

    // /// Test against the example.
    // #[test]
    // fn part2_example01() {
    //     assert_eq!(part2("example1.txt"), 25272);
    // }
    //
    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2("input.txt"), 6934702555);
    // }

    #[test]
    fn test_calculate_area() {
        let c1 = Coord::new(2, 5);
        let c2 = Coord::new(9, 7);
        assert_eq!(calculate_area(&c1, &c2), 24);

        let c1 = Coord::new(7, 1);
        let c2 = Coord::new(11, 7);
        assert_eq!(calculate_area(&c1, &c2), 35);

        let c1 = Coord::new(7, 3);
        let c2 = Coord::new(2, 3);
        assert_eq!(calculate_area(&c1, &c2), 6);

        let c1 = Coord::new(2, 5);
        let c2 = Coord::new(11, 1);
        assert_eq!(calculate_area(&c1, &c2), 50);
    }
}
