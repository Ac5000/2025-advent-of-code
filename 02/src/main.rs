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
    /// Chunk sizes.
    invalid_ids2: Vec<u64>,
}

impl ProductID {
    /// Make a new ProductID from two numbers.
    fn new(start: u64, end: u64) -> Self {
        Self {
            start,
            end,
            invalid_ids: Vec::new(),
            invalid_ids2: Vec::new(),
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

    /// Sum the Invalid IDs for getting final answer.
    fn sum_invalids2(&self) -> u64 {
        let mut sum: u64 = 0;
        for id in &self.invalid_ids2 {
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
        for sequence in self.start..=self.end {
            if check_invalid(sequence) {
                self.push_invalid(sequence);
            }
        }
    }

    /// Run through all the ProductIDs and add the invalids to the vec.
    fn find_invalids2(&mut self) {
        for sequence in self.start..=self.end {
            // Once we confirm the number is invalid, skip checking other chunk_sizes.
            let mut skip_ahead = false;
            let chunk_sizes = get_chunk_sizes(sequence);
            for chunk_size in chunk_sizes {
                if skip_ahead {
                    break;
                };
                let chunks = get_chunks(sequence, chunk_size);
                if check_invalid2(&chunks) {
                    self.invalid_ids2.push(sequence);
                    skip_ahead = true;
                }
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

/// Checks all the Vecs match. Returns true if invalid.
fn check_invalid2(chunks: &Vec<Vec<char>>) -> bool {
    for chunk in chunks {
        if chunk != &chunks[0] {
            return false;
        }
    }
    true
}

/// Figure out what chunks we can split the number's digits in to equally.
fn get_chunk_sizes(number: u64) -> Vec<u32> {
    // Get number of digits in the number.
    let num_digits = number.checked_ilog10().unwrap_or(0) + 1;
    // Get half the number of digits to figure out how far to iterate.
    let half_digits = num_digits / 2;
    // Figure out what sized chunks we can split into by checking remainders.
    let mut chunks: Vec<u32> = Vec::new();
    for chunk in 1..=half_digits {
        if num_digits % chunk != 0 {
            continue;
        }
        chunks.push(chunk);
    }
    chunks
}

/// Given the number and chunk size, split the number into chunks of chars.
fn get_chunks(number: u64, chunk_size: u32) -> Vec<Vec<char>> {
    let mut ret: Vec<Vec<char>> = Vec::new();
    // Get initial Vec of chars from the number.
    let mut characters: Vec<char> = number.to_string().chars().collect();
    let number_of_chunks = characters.len() / chunk_size as usize;
    let (mut left, mut right) = characters.split_at_mut(0);
    // Left should be empty here. Just using it to initilize the variable. The assert
    // gets rid of the warning.
    assert_eq!(left, []);
    for _ in 0..number_of_chunks {
        (left, right) = right.split_at_mut(chunk_size as usize);
        ret.push(left.to_vec());
    }
    ret
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

/// Add all invalid IDs together. Invalid IDs are sequences of digits that repeat
/// at least twice.
fn part2(file_name: &str) -> u64 {
    let file_contents = std::fs::read_to_string(file_name).expect("Couldn't open file");
    let mut product_ids = parse_text(&file_contents);
    let mut sum = 0;
    for id in product_ids.iter_mut() {
        id.find_invalids2();
        sum += id.sum_invalids2();
    }
    sum
}

/// Main function / code entry point.
fn main() {
    println!("Sum of invalid IDs for example1: {}", part1("example1.txt"));
    println!("Sum of invalid IDs for input: {}", part1("input.txt"));
    println!(
        "Sum of invalid IDs for part 2 example1: {}",
        part2("example1.txt")
    );
    println!(
        "Sum of invalid IDs for part 2 input: {}",
        part1("input.txt")
    );
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

    /// Test against the example.
    #[test]
    fn part2_example01() {
        assert_eq!(part2("example1.txt"), 4174379265);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("input.txt"), 22471660255);
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

    /// Struggled on part2 to get the example result and had to do each example
    /// individually.
    #[test]
    fn test_find_invalids2() {
        let mut id = ProductID::new(11, 22);
        id.find_invalids2();
        assert_eq!(id.invalid_ids2, vec![11, 22]);
        let mut id = ProductID::new(95, 115);
        id.find_invalids2();
        assert_eq!(id.invalid_ids2, vec![99, 111]);
        let mut id = ProductID::new(998, 1012);
        id.find_invalids2();
        assert_eq!(id.invalid_ids2, vec![999, 1010]);
        let mut id = ProductID::new(1188511880, 1188511890);
        id.find_invalids2();
        assert_eq!(id.invalid_ids2, vec![1188511885]);
        let mut id = ProductID::new(222220, 222224);
        id.find_invalids2();
        assert_eq!(id.invalid_ids2, vec![222222]);
        let mut id = ProductID::new(1698522, 1698528);
        id.find_invalids2();
        assert_eq!(id.invalid_ids2, vec![]);
    }

    #[test]
    fn test_get_chunk_sizes() {
        let number = 1;
        assert_eq!(get_chunk_sizes(number), vec![]);
        let number = 11;
        assert_eq!(get_chunk_sizes(number), vec![1]);
        let number = 111;
        assert_eq!(get_chunk_sizes(number), vec![1]);
        let number = 1111;
        assert_eq!(get_chunk_sizes(number), vec![1, 2]);
    }

    #[test]
    fn test_get_chunks() {
        assert_eq!(get_chunks(11, 1), vec![vec!['1'], vec!['1']]);
        assert_eq!(get_chunks(12, 1), vec![vec!['1'], vec!['2']]);
        assert_eq!(
            get_chunks(1111, 1),
            vec![vec!['1'], vec!['1'], vec!['1'], vec!['1']]
        );
        assert_eq!(get_chunks(1111, 2), vec![vec!['1', '1'], vec!['1', '1']]);
    }

    #[test]
    fn test_check_invalid2() {
        assert_eq!(check_invalid2(&vec![vec!['1'], vec!['1']]), true);
        assert_eq!(check_invalid2(&vec![vec!['2'], vec!['1']]), false);
        assert_eq!(check_invalid2(&vec![vec!['1', '1'], vec!['1', '1']]), true);
        assert_eq!(
            check_invalid2(&vec![vec!['1'], vec!['1'], vec!['1'], vec!['1']]),
            true
        );
    }
}
