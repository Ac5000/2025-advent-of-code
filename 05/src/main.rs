//! Day 05: Cafeteria

use std::{collections::VecDeque, ops::RangeInclusive};

/// Parse the incoming file to Vec of ranges and Vec of IDs.
fn parse_text(string: &String) -> (VecDeque<RangeInclusive<u64>>, Vec<u64>) {
    // Using VecDeque for its rotate ability on part 2.
    let mut ranges: VecDeque<RangeInclusive<u64>> = VecDeque::new();
    let mut ids: Vec<u64> = Vec::new();

    for line in string.lines() {
        // Range
        if line.trim().contains('-') {
            let (start_raw, end_raw) = line.split_once('-').expect("Failed to split id_range.");
            let start = u64::from_str_radix(start_raw, 10)
                .expect(&format!("Failed to convert {start_raw} to u64."));
            let end = u64::from_str_radix(end_raw, 10)
                .expect(&format!("Failed to convert {end_raw} to u64."));
            ranges.push_back(RangeInclusive::new(start, end));
        // Ingredient ID if not the empty line.
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

/// Determine if the ranges overlap.
fn ranges_overlap(range1: &RangeInclusive<u64>, range2: &RangeInclusive<u64>) -> bool {
    if range1.contains(range2.start()) || range1.contains(range2.end()) {
        return true;
    }
    if range2.contains(range1.start()) || range2.contains(range1.end()) {
        return true;
    }
    false
}

/// Merge two ranges that are known to overlap.
fn merge_ranges(range1: &RangeInclusive<u64>, range2: &RangeInclusive<u64>) -> RangeInclusive<u64> {
    let start = if range1.start() <= range2.start() {
        range1.start()
    } else {
        range2.start()
    };

    let end = if range1.end() >= range2.end() {
        range1.end()
    } else {
        range2.end()
    };

    RangeInclusive::new(*start, *end)
}

/// Flatten the collection of ranges.
/// Learned I can't just use a flatten().collect() on my Vec when it tried to
/// allocate 9896 Gigs of memory...
fn part2(file_name: &str) -> usize {
    let file_contents = std::fs::read_to_string(file_name).expect("Couldn't open file");
    let (mut ranges, _) = parse_text(&file_contents);
    let mut outer_count = 0;
    let mut change_made = false;

    // Keep looping the ranges until we don't do anything for a loop.
    loop {
        // No changes made. Increment the outer loop count.
        if !change_made {
            outer_count += 1;
        // Change made, reset outer counter.
        } else {
            outer_count = 0;
        };

        // Reset the flag for this loop.
        change_made = false;

        // Made the full loop without changing something, break out.
        if outer_count > ranges.len() {
            break;
        };

        // Pop range off the stack to use to compare against.
        let range1 = ranges.pop_front().expect("Failed to pop range1.");

        // Keep looping the ranges until we don't merge something for a loop.
        let mut inner_count = 0;
        loop {
            // No changes made. Increment the inner loop count.
            if !change_made {
                inner_count += 1;
            } else {
                change_made = false;
            };

            // Made the full loop without changing something, put range1 back and
            // break out.
            if inner_count > ranges.len() {
                ranges.push_back(range1);
                break;
            };

            // If the ranges don't overlap, rotate ranges for next inner loop.
            if !ranges_overlap(
                &range1,
                ranges.front().expect("failed to get front of ranges."),
            ) {
                ranges.rotate_left(1);
                change_made = false;
            // If they do overlap, pop it from stack, merge, then push merge onto
            // stack. Then we break this loop so we can get a new range1. Otherwise
            // our new merged range will overlap and we'd get stuck in loop. (Ask
            // me how I know...)
            } else {
                let range2 = &ranges.pop_front().expect("Failed to pop range2.");
                ranges.push_back(merge_ranges(&range1, range2));
                change_made = true;
                break;
            }
        }
    }

    // Sum up the count for the ranges to get the final answer.
    let mut sum = 0;
    for range in ranges {
        sum += range.count();
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
        assert_eq!(part1("example1.txt"), 3);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1("input.txt"), 617);
    }

    /// Test against the example.
    #[test]
    fn part2_example01() {
        assert_eq!(part2("example1.txt"), 14);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("input.txt"), 338258295736104);
    }

    #[test]
    fn test_ranges_overlap() {
        assert_eq!(
            ranges_overlap(&RangeInclusive::new(10, 14), &RangeInclusive::new(12, 18)),
            true
        );
        assert_eq!(
            ranges_overlap(&RangeInclusive::new(10, 14), &RangeInclusive::new(16, 20)),
            false
        );
    }

    #[test]
    fn test_merge_ranges() {
        assert_eq!(
            merge_ranges(&RangeInclusive::new(10, 14), &RangeInclusive::new(12, 18)),
            RangeInclusive::new(10, 18)
        );
        assert_eq!(
            merge_ranges(&RangeInclusive::new(12, 18), &RangeInclusive::new(10, 14)),
            RangeInclusive::new(10, 18)
        );
    }
}
