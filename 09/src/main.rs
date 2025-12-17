//! Day 09: Movie Theater

use std::iter;
use utilities::{Coord, coord};

/// Parse the incoming file to Vec of Coords.
fn parse_text(string: &String) -> Vec<Coord> {
    let mut coords: Vec<Coord> = Vec::new();

    for line in string.lines() {
        let v: Vec<&str> = line.split(',').collect();
        coords.push(coord!(
            v[0].parse().expect("X Failed to parse."),
            v[1].parse().expect("Y Failed to parse.")
        ));
    }
    coords
}

/// Find the biggest area between Coords
fn get_biggest_area(coords: &Vec<Coord>) -> u64 {
    let areas = get_areas(coords);
    println!("get_biggest_area checked {} combinations.", areas.len());

    // Grab first since areas is already sorted by area.
    areas
        .first()
        .expect("get_biggest_area: area.first() is empty.")
        .0
}

/// Returns all the areas with the two Coord that made it, sorted by area.
fn get_areas(coords: &Vec<Coord>) -> Vec<(u64, Coord, Coord)> {
    let area_iter = (0..coords.len()).flat_map(move |a| {
        (a + 1..coords.len()).map(move |b| {
            let coord_a = coords[a];
            let coord_b = coords[b];
            let area = calculate_area(&coord_a, &coord_b);
            (area, coord_a, coord_b)
        })
    });
    let mut areas: Vec<_> = area_iter.collect();
    // Sort goes low->high, so we reverse it right afterwards.
    areas.sort_by_key(|&(area, _, _)| area);
    areas.reverse();

    areas
}

/// Find the biggest area between the red tiles that form a rectangle within the
/// polygon.
fn get_biggest_area_filtered(red_tiles: &Vec<Coord>) -> u64 {
    let areas = get_areas(red_tiles);

    // Iterate through the areas with a find. Find short circuits to return first
    // thing that returns true. Since areas is already sorted by max, first item
    // find returns will be the largest area. We try to "find" an area that
    // doesn't intersect so we can avoid checking the rest of the areas. (Like I
    // tried the first time...)
    areas
        .iter()
        .find(|(_area, a, b)| {
            // Get the min and max bounds for the two coordinates for this area.
            let bounds = bounds(a, b);
            // First part of this iterator is just the insanity needed to iterate
            // like a "window(2)" would with a slice. The iter:once's are there
            // so we can see the (last,first) combination of red_tiles.
            !red_tiles
                .iter()
                .chain(iter::once(&red_tiles[0]))
                .zip(red_tiles.iter().chain(iter::once(&red_tiles[0])).skip(1))
                // Any short circuits if it ever gets true. Saves iterations for
                // lines that intersect the bounds of the area. Each "line" is two
                // Coords from the red_tiles.
                .any(|line| intersecting_line(line, bounds))
        })
        .expect("Iterator in get_biggest_area_filtered failed to get a value.")
        .0
}

/// Gets the bounds for the two coordinates as a tuple (x,y) where x=min and
/// y=max
fn bounds(a: &Coord, b: &Coord) -> (Coord, Coord) {
    let min = Coord::new(a.x.min(b.x), a.y.min(b.y));
    let max = Coord::new(a.x.max(b.x), a.y.max(b.y));
    (min, max)
}

/// Checks if the line, represented by two Coords (line_coord1,line_coord2),
/// intersects with the bounds (min,max) of the area. Returns true if it intersects.
/// Big thanks to https://github.com/connorslade/advent-of-code/blob/main/aoc_2025/src/day_09.rs
/// because I definitely struggled figuring this out...
fn intersecting_line(
    (line_coord1, line_coord2): (&Coord, &Coord),
    (min, max): (Coord, Coord),
) -> bool {
    let (line_min, line_max) = bounds(line_coord1, line_coord2);

    // Horizontal Line
    line_coord1.x == line_coord2.x 
        // Line's Y bounds within area's Y bounds.
        && (((line_min.y < min.y && line_max.y > min.y) 
            || (line_min.y < max.y && line_max.y > max.y)
            || (line_min.y >= min.y && line_max.y <= max.y))
            // Line's X coordinate within the area's X bounds.
            && line_coord1.x > min.x
            && line_coord1.x < max.x)
        // Vertical Line
        || line_coord1.y == line_coord2.y 
            // Line's X bounds within area's X bounds.
            && (((line_min.x < min.x && line_max.x > min.x)
                || (line_min.x < max.x && line_max.x > max.x)
                || (line_min.x >= min.x && line_max.x <= max.x))
                // Line's Y coordinate within the area's Y bounds.
                && line_coord1.y > min.y
                && line_coord1.y < max.y)
}

/// Calculate the area of the rectangle between these two points.
fn calculate_area(coord1: &Coord, coord2: &Coord) -> u64 {
    // +1s necessary since area includes the coords locations.
    let x_axis = u64::from(coord1.x.abs_diff(coord2.x) + 1);
    let y_axis = u64::from(coord1.y.abs_diff(coord2.y) + 1);
    x_axis * y_axis
}

/// Find the largest rectangle by area from pairs of points.
fn part1(file_name: &str) -> u64 {
    let file_contents = std::fs::read_to_string(file_name).expect("Couldn't open file");
    let tiles = parse_text(&file_contents);
    get_biggest_area(&tiles)
}

/// Find the largest rectangle by area from pairs of points. All of rectangle must
/// be within the polygon made from the total collection of points.
fn part2(file_name: &str) -> u64 {
    let file_contents = std::fs::read_to_string(file_name).expect("Couldn't open file");
    let tiles = parse_text(&file_contents);
    get_biggest_area_filtered(&tiles)
}

/// Main function / code entry point.
fn main() {
    println!("Sum for example1: {}", part1("example1.txt"));
    println!("Sum for input: {}", part1("input.txt"));
    println!("Sum for example1 part2: {}", part2("example1.txt"));
    println!("Sum for input part2: {}", part2("input.txt"));
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
        let part1 = part1("input.txt");
        assert!(part1 > 4225115601); // First attempt too low.
        assert_eq!(part1, 4759420470);
    }

    /// Test against the example.
    #[test]
    fn part2_example01() {
        assert_eq!(part2("example1.txt"), 24);
    }

    #[test]
    fn test_part2() {
        let part2 = part2("input.txt");
        assert!(part2 > 19851568); // First attempt too low. Also took > 10 minutes..
        assert_eq!(part2, 1603439684);
    }

    #[test]
    fn test_calculate_area() {
        let c1 = coord!(2, 5);
        let c2 = coord!(9, 7);
        assert_eq!(calculate_area(&c1, &c2), 24);

        let c1 = coord!(7, 1);
        let c2 = coord!(11, 7);
        assert_eq!(calculate_area(&c1, &c2), 35);

        let c1 = coord!(7, 3);
        let c2 = coord!(2, 3);
        assert_eq!(calculate_area(&c1, &c2), 6);

        let c1 = coord!(2, 5);
        let c2 = coord!(11, 1);
        assert_eq!(calculate_area(&c1, &c2), 50);
    }
}
