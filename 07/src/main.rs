//! Day 07: Laboratories

use std::collections::HashMap;

use utilities::{
    coord::Coord,
    grid::{Grid, iter_char_map_keys},
};

/// Check if the beam is above this coordinate. E.g., do we care about this coord?
fn is_beam_above(coord: &Coord, grid: &Grid) -> bool {
    if grid
        .char_map
        .get(&coord.north())
        .is_some_and(|&x| x == '|' || x == 'S')
    {
        true
    } else {
        false
    }
}

/// Check if this Coord will split the beam.
fn check_split(coord: &Coord, grid: &Grid) -> bool {
    if grid.char_map.get(coord).is_some_and(|&x| x == '^') {
        if is_beam_above(coord, grid) {
            return true;
        }
    }
    false
}

/// Splits the beam at the given coordinate by placing '|' on left/right.
fn split_beam(coord: &Coord, grid: &mut Grid) {
    let left = coord.west();
    let right = coord.east();
    grid.char_map.entry(left).and_modify(|x| *x = '|');
    grid.char_map.entry(right).and_modify(|x| *x = '|');
}

/// Check if the beam should continue.
fn check_continues(coord: &Coord, grid: &Grid) -> bool {
    if grid
        .char_map
        .get(coord)
        .is_some_and(|&x| x == '.' || x == '|')
    {
        if is_beam_above(coord, &grid) {
            return true;
        }
    }
    false
}

/// Continue the beam through this coordinate.
fn continue_beam(coord: Coord, grid: &mut Grid) {
    grid.char_map.entry(coord).and_modify(|x| *x = '|');
}

/// Figure out how many times the beam splits.
fn part1(file_name: &str) -> u64 {
    let mut grid = Grid::new_from_file(file_name);
    let mut splits = 0;
    for coord in iter_char_map_keys(grid.max_x, grid.max_y) {
        if !is_beam_above(&coord, &grid) {
            continue;
        }
        if check_continues(&coord, &grid) {
            continue_beam(coord, &mut grid);
        }
        if check_split(&coord, &grid) {
            split_beam(&coord, &mut grid);
            splits += 1;
        }
    }
    splits
}

/// At each split, create two new "timelines" where each has the beam take a different
/// path. Sum the amount of timelines.
fn part2(file_name: &str) -> u64 {
    let mut grid = Grid::new_from_file(file_name);
    let mut summing_grid: HashMap<Coord, u64> = HashMap::new();

    // Use the same splitting logic from part1, but keep a second grid of values.
    // These values "waterfall" down and sum when they combine. The values represent
    // how many possible paths led to that coordinate.
    for coord in iter_char_map_keys(grid.max_x, grid.max_y) {
        if !is_beam_above(&coord, &grid) {
            continue;
        }
        if check_continues(&coord, &grid) {
            continue_beam(coord, &mut grid);
            let north_val = *summing_grid.get(&coord.north()).unwrap_or(&1);
            summing_grid
                .entry(coord)
                .and_modify(|x| *x += north_val)
                .or_insert(north_val);
        }
        if check_split(&coord, &grid) {
            split_beam(&coord, &mut grid);
            let north_val = *summing_grid.get(&coord.north()).unwrap_or(&1);
            summing_grid
                .entry(coord.east())
                .and_modify(|x| *x += north_val)
                .or_insert(north_val);
            summing_grid
                .entry(coord.west())
                .and_modify(|x| *x += north_val)
                .or_insert(north_val);
        }
    }

    // Sum the last row to get the final amount of timelines.
    let mut sum = 0;
    for x in 0..=grid.max_x {
        let val = summing_grid.get(&Coord::new(x, grid.max_y)).unwrap_or(&0);
        sum += val
    }

    sum
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
        assert_eq!(part1("example1.txt"), 21);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1("input.txt"), 1649);
    }

    /// Test against the example.
    #[test]
    fn part2_example01() {
        assert_eq!(part2("example1.txt"), 40);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("input.txt"), 16937871060075);
    }
}
