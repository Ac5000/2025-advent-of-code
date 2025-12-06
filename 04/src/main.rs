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
fn get_accessible_papers(papers: &HashSet<Coord>) -> HashSet<Coord> {
    papers
        .into_iter()
        .filter(|&&x| x.get_surrounding_coords().intersection(&papers).count() < 4)
        .copied()
        .collect()
}

/// Remove the accessible_papers from the papers.
fn remove_papers(papers: &mut HashSet<Coord>, accessible_papers: &HashSet<Coord>) {
    for paper in accessible_papers {
        papers.remove(&paper);
    }
}

/// Find all @ locations with less than 4 @'s around them.
fn part1(file_name: &str) -> usize {
    let grid = Grid::new_from_file(file_name);
    let papers = get_papers(&grid);
    let accessible_papers = get_accessible_papers(&papers);
    accessible_papers.len()
}

/// Do part1 until you can't do it anymore.
fn part2(file_name: &str) -> usize {
    let grid = Grid::new_from_file(file_name);
    let mut papers = get_papers(&grid);
    let mut sum = 0;
    loop {
        let accessible_papers = get_accessible_papers(&papers);
        if accessible_papers.len() <= 0 {
            break;
        }
        sum += accessible_papers.len();
        remove_papers(&mut papers, &accessible_papers);
    }
    sum
}

/// Main function / code entry point.
fn main() {
    println!("Sum for example1: {}", part1("example1.txt"));
    println!("Sum for input: {}", part1("input.txt"));
    println!("Sum for example1: {}", part2("example1.txt"));
    println!("Sum for input: {}", part2("input.txt"));
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

    /// Test against the example.
    #[test]
    fn part2_example01() {
        assert_eq!(part2("example1.txt"), 43);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("input.txt"), 9397);
    }

    #[test]
    fn test_get_papers() {
        let example_grid = Grid::new_from_file("example1.txt");
        let papers = get_papers(&example_grid);
        assert!(papers.contains(&Coord::new(0, 2)));
        assert!(papers.contains(&Coord::new(8, 9)));
        assert!(!papers.contains(&Coord::new(0, 0)));
    }
}
