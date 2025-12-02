//! Day 02: Gift Shop

/// Struct representing an ID range.
#[derive(Debug, Eq, Hash, PartialEq)]
struct ProductID {
    /// Start of the ID range.
    start: u64,
    /// End of the ID range.
    end: u64,
    /// Invalid IDs.
    invalid_ids: Vec<u64>,
}

impl ProductID {
    /// Make a new ProductID from two numbers.
    fn new(start: u64, end: u64) -> Self {
        Self {
            start,
            end,
            invalid_ids: Vec::new(),
        }
    }

    /// Sum the Invalid IDs for getting final answer.
    fn sum_invalids(&self) -> u64 {
        let mut sum: u64 = 0;
        for id in &self.invalid_ids {
            sum = sum + id
        }
        sum
    }
}

/// Parse the incoming file to Vec of ProductID's
fn parse_text(string: &String) -> Vec<ProductID> {
    let mut ret: Vec<ProductID> = Vec::new();
    for id_range in string.trim().split(',') {
        let (start_raw, end_raw) = id_range.split_once('-').expect("Failed to split id_range.");
        let start = u64::from_str_radix(start_raw, 10)
            .expect(&format!("Failed to convert {start_raw} to u32."));
        let end = u64::from_str_radix(end_raw, 10)
            .expect(&format!("Failed to convert {end_raw} to u32."));
        ret.push(ProductID::new(start, end));
    }
    ret
}

/// Add all invalid IDs together. Invalid IDs are sequences of digits that repeat
/// twice.
fn part1(file_name: &str) -> u64 {
    let file_contents = std::fs::read_to_string(file_name).expect("Couldn't open file");
    let product_ids = parse_text(&file_contents);
    let mut sum = 0;
    for id in product_ids {
        sum = sum + id.sum_invalids();
    }
    sum
}

/// Main function / code entry point.
fn main() {
    println!("Sum of invalid IDs for example1: {}", part1("example1.txt"));
    println!("Sum of invalid IDs for input: {}", part1("input.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test against the example.
    #[test]
    fn part1_example01() {
        assert_eq!(part1("example1.txt"), 1227775554);
    }

    /// Test for part 1.
    #[test]
    fn test_part1() {
        assert_eq!(part1("input.txt"), 1227775554);
    }

    /// Test parse_text.
    #[test]
    fn test_parse_text() {
        let input = "11-22,95-115".to_string();
        let expected = vec![ProductID::new(11, 22), ProductID::new(95, 115)];
        assert_eq!(parse_text(&input), expected);
    }

    /// Test sum_invalids.
    #[test]
    fn test_sum_invalids() {
        let mut id = ProductID::new(11, 22);
        id.invalid_ids = vec![11, 22];
        assert_eq!(id.sum_invalids(), 33);
    }
}
