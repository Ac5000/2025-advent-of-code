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
            sum += id
        }
        sum
    }

    /// Adds the invalid id to the struct.
    fn push_invalid(&mut self, number: u64) {
        self.invalid_ids.push(number);
    }

    /// Run through all the ProductIDs and add the invalids to the vec.
    fn find_invalids(&mut self) {
        for sequence in self.start..self.end + 1 {
            if check_invalid(sequence) {
                self.push_invalid(sequence);
            }
        }
    }
}

/// Check if number has even number of digits
fn even_digits(number: u64) -> bool {
    let num_digits = number.checked_ilog10().unwrap_or(0) + 1;
    if num_digits.rem_euclid(2) == 0 {
        true
    } else {
        false
    }
}

/// Checks if number is invalid by turning it into a vec of chars and splitting
/// the vec in two. If the vecs match, it is invalid. Returns true if invalid.
fn check_invalid(number: u64) -> bool {
    if !even_digits(number) {
        return false;
    };
    let mut characters1: Vec<char> = number.to_string().chars().collect();
    let characters2 = characters1.split_off(characters1.len() / 2);
    characters1 == characters2
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
    let mut product_ids = parse_text(&file_contents);
    let mut sum = 0;
    for id in product_ids.iter_mut() {
        id.find_invalids();
        sum += id.sum_invalids();
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

    #[test]
    fn test_part1() {
        assert_eq!(part1("input.txt"), 13108371860);
    }

    #[test]
    fn test_parse_text() {
        let input = "11-22,95-115".to_string();
        let expected = vec![ProductID::new(11, 22), ProductID::new(95, 115)];
        assert_eq!(parse_text(&input), expected);
    }

    #[test]
    fn test_sum_invalids() {
        let mut id = ProductID::new(11, 22);
        id.invalid_ids = vec![11, 22];
        assert_eq!(id.sum_invalids(), 33);
    }

    #[test]
    fn test_even_digits() {
        assert_eq!(even_digits(5), false);
        assert_eq!(even_digits(10), true);
        assert_eq!(even_digits(101), false);
    }

    #[test]
    fn test_check_invalid() {
        assert_eq!(check_invalid(1), false);
        assert_eq!(check_invalid(10), false);
        assert_eq!(check_invalid(11), true);
        assert_eq!(check_invalid(111), false);
    }

    #[test]
    fn test_find_invalids() {
        let mut id = ProductID::new(11, 22);
        id.find_invalids();
        assert_eq!(id.invalid_ids, vec![11, 22]);
    }
}
