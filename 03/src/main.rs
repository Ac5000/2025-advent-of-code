//! Day 03: Lobby

/// Struct representing a battery bank.
#[derive(Debug, Eq, Hash, PartialEq)]
struct Bank {
    /// Battery joltages.
    joltages: Vec<u8>,
}

impl Bank {
    /// Make a new Bank from Vec of u8's.
    fn new(joltages: Vec<u8>) -> Self {
        Self { joltages }
    }

    /// Find the largest joltage in the range provided. (Jolts,pos)
    fn find_largest_in_range(&self, start: usize, end: usize) -> (u8, usize) {
        let mut largest = u8::MIN;
        let mut largest_pos = usize::MIN;
        for (pos, joltage) in self.joltages[start..=end].iter().enumerate() {
            if joltage > &largest {
                largest = *joltage;
                largest_pos = pos + start;
            }

            // Stop iterating if we found a 9 since can't be higher.
            if largest == 9 {
                break;
            }
        }
        (largest, largest_pos)
    }

    /// Find the largest joltage given the number of digits/batteries to use.
    fn find_largest_by_num_digits(&self, digits: usize) -> u64 {
        let mut ordered_digits: Vec<u8> = Vec::with_capacity(digits);
        let mut start = 0;
        for digit in (0..digits).rev() {
            let (num, pos) = self.find_largest_in_range(start, self.joltages.len() - 1 - digit);
            start = pos + 1;
            ordered_digits.push(num);
        }

        combine_u8s_to_u64(ordered_digits)
    }
}

/// Take a vec of u8's and combine them into a single number. Vec[0] = 1's digit.
fn combine_u8s_to_u64(digits: Vec<u8>) -> u64 {
    let mut ret: u64 = 0;
    for (pos, digit) in digits.into_iter().rev().enumerate() {
        let digit = digit;
        ret += digit as u64 * 10_u64.pow(pos as u32);
    }
    ret
}

/// Parse the incoming file to Vec of Bank's
fn parse_text(string: &String) -> Vec<Bank> {
    let mut ret: Vec<Bank> = Vec::new();
    for line in string.lines() {
        let joltages: Vec<u8> = line
            .trim()
            .chars()
            .map(|f| {
                u8::try_from(
                    f.to_digit(10)
                        .expect(&format!("Failed to convert {f} to u32/digit.")),
                )
                .expect(&format!("Failed to convert {f} to u8."))
            })
            .collect();
        ret.push(Bank::new(joltages));
    }
    ret
}

/// For each bank, get the largest 2 digit number. Digit ordering dictated by order
/// in the bank. (E.g. 1's digit must come after 10's.)
fn part1(file_name: &str) -> u64 {
    let file_contents = std::fs::read_to_string(file_name).expect("Couldn't open file");
    let banks = parse_text(&file_contents);
    let mut sum = 0;
    for bank in banks {
        sum += bank.find_largest_by_num_digits(2);
    }
    sum
}

/// For each bank, get the largest 12 digit number. Digit ordering dictated by order
/// in the bank. (E.g. 1's digit must come after 10's.)
fn part2(file_name: &str) -> u64 {
    let file_contents = std::fs::read_to_string(file_name).expect("Couldn't open file");
    let banks = parse_text(&file_contents);
    let mut sum = 0;
    for bank in banks {
        sum += bank.find_largest_by_num_digits(12);
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
        assert_eq!(part1("example1.txt"), 357);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1("input.txt"), 17311);
    }

    /// Test against the example.
    #[test]
    fn part2_example01() {
        assert_eq!(part2("example1.txt"), 3121910778619);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("input.txt"), 171419245422055);
    }

    #[test]
    fn test_parse_text() {
        let input = "9876".to_string();
        let expected = vec![Bank::new(vec![9, 8, 7, 6])];
        assert_eq!(parse_text(&input), expected);
    }

    #[test]
    fn test_find_largest_in_range() {
        let bank = Bank::new(vec![9, 8, 7, 6, 9]);
        assert_eq!(bank.find_largest_in_range(1, 2), (8, 1));
    }

    #[test]
    fn test_combine_u8s_to_u64() {
        assert_eq!(combine_u8s_to_u64(vec![1, 2, 3, 4]), 1234)
    }

    #[test]
    fn test_find_largest_by_num_digits() {
        let bank = Bank::new(vec![9, 8, 7, 6, 9]);
        assert_eq!(bank.find_largest_by_num_digits(2), 99);
        let bank = Bank::new(vec![2, 3, 4, 2, 7, 8]);
        assert_eq!(bank.find_largest_by_num_digits(3), 478);
    }
}
