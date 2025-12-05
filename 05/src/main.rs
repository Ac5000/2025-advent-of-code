//! Day 05: Cafeteria

use std::ops::RangeInclusive;

/// Parse the incoming file to Vec of ranges and Vec of IDs.
fn parse_text(string: &String) -> (Vec<RangeInclusive<u64>>, Vec<u64>) {
    let mut ranges: Vec<RangeInclusive<u64>> = Vec::new();
    let mut ids: Vec<u64> = Vec::new();

    for line in string.lines() {
        if line.trim().contains('-') {
            let (start_raw, end_raw) = line.split_once('-').expect("Failed to split id_range.");
            let start = u64::from_str_radix(start_raw, 10)
                .expect(&format!("Failed to convert {start_raw} to u64."));
            let end = u64::from_str_radix(end_raw, 10)
                .expect(&format!("Failed to convert {end_raw} to u64."));
            ranges.push(RangeInclusive::new(start, end));
        } else if line.trim().len() > 0 {
            ids.push(
                u64::from_str_radix(line.trim(), 10)
                    .expect(&format!("Failed to convert {line} to u64.")),
            );
        }
    }
    (ranges, ids)
}

/// Find ids not in the collection of ranges.
fn part1(file_name: &str) -> u32 {
    let file_contents = std::fs::read_to_string(file_name).expect("Couldn't open file");
    let (ranges, ids) = parse_text(&file_contents);
    let mut fresh_ingredients: u32 = 0;
    for id in &ids {
        let mut found_already = false;
        for range in &ranges {
            if found_already {
                break;
            }
            if range.contains(id) {
                fresh_ingredients += 1;
                found_already = true;
            }
        }
    }
    fresh_ingredients
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
        assert_eq!(part1("example1.txt"), 3);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1("input.txt"), 617);
    }

    // /// Test against the example.
    // #[test]
    // fn part2_example01() {
    //     assert_eq!(part2("example1.txt"), 43);
    // }
    //
    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2("input.txt"), 9397);
    // }
}
