//! Day 04: Printing Department

use std::collections::HashSet;
use utilities::{coord::Coord, grid::Grid};

/// Get all the locations of paper.
fn get_papers(grid: &Grid) -> HashSet<Coord> {
    let mut papers = HashSet::new();
    for (coord, character) in grid.char_map.iter() {
        if character == &'@' {
            papers.insert(*coord);
        }
    }
    papers
}

/// Get all papers with fewer than 4 papers nearby.
fn get_accessible_papers(papers: &HashSet<Coord>) -> HashSet<&Coord> {
    let mut accessible = HashSet::new();
    for paper in papers {
        if paper
            .get_surrounding_coords()
            .intersection(papers)
            .collect::<HashSet<&Coord>>()
            .len()
            < 4
        {
            accessible.insert(paper);
        }
    }

    accessible
}

/// Find all @ locations with less than 4 @'s around them.
fn part1(file_name: &str) -> u32 {
    let grid = Grid::new_from_file(file_name);
    let papers = get_papers(&grid);
    let accessible_papers = get_accessible_papers(&papers);
    accessible_papers.len() as u32
}

/// Main function / code entry point.
fn main() {
    println!("Sum for example1: {}", part1("example1.txt"));
    println!("Sum for input: {}", part1("input.txt"));
    // println!("Sum for example1: {}", part2("example1.txt"));
    // println!("Sum for input: {}", part2("input.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test against the example.
    #[test]
    fn part1_example01() {
        assert_eq!(part1("example1.txt"), 13);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1("input.txt"), 1604);
    }

    // /// Test against the example.
    // #[test]
    // fn part2_example01() {
    //     assert_eq!(part2("example1.txt"), 3121910778619);
    // }
    //
    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2("input.txt"), 171419245422055);
    // }

    #[test]
    fn test_get_papers() {
        let example_grid = Grid::new_from_file("example1.txt");
        let papers = get_papers(&example_grid);
        assert!(papers.contains(&Coord::new(0, 2)));
        assert!(papers.contains(&Coord::new(8, 9)));
        assert!(!papers.contains(&Coord::new(0, 0)));
    }
}
