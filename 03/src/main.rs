//! Day 03: Lobby

/// Struct representing a battery bank.
#[derive(Debug, Eq, Hash, PartialEq)]
struct Bank {
    /// Battery joltages.
    joltages: Vec<u8>,
    /// Largest first battery.
    first: u8,
    /// Largest first battery position.
    first_pos: usize,
    /// Largest second battery.
    second: u8,
    /// Largest second battery position.
    second_pos: usize,
}

impl Bank {
    /// Make a new Bank from Vec of u8's.
    fn new(joltages: Vec<u8>) -> Self {
        Self {
            joltages,
            first: 0,
            first_pos: 0,
            second: 0,
            second_pos: 0,
        }
    }

    /// Find the largest joltages and their positions.
    fn find_largest_joltages(&mut self) -> u32 {
        let mut largest = u32::MIN;
        let mut largest_first = u8::MIN;
        let mut first_pos: usize = 0;
        let mut largest_second = u8::MIN;
        let mut second_pos: usize = 0;
        let end = self.joltages.len();
        for (pos1, joltage1) in self.joltages.iter().enumerate() {
            let second_range = &self.joltages[pos1 + 1..end];
            for (pos2, joltage2) in second_range.iter().enumerate() {
                if two_u8_to_u32(*joltage1, *joltage2) > largest {
                    largest_first = *joltage1;
                    first_pos = pos1;
                    largest_second = *joltage2;
                    second_pos = pos2 + pos1 + 1; // since second loop starts here.
                    largest = two_u8_to_u32(largest_first, largest_second);
                }
            }

            // Stop iterating if we found a 99 since can't be higher.
            if largest == 99 {
                break;
            }
        }
        self.first = largest_first;
        self.first_pos = first_pos;
        self.second = largest_second;
        self.second_pos = second_pos;
        largest
    }
}

/// Take 2 u8's and turn into u32 where one = 10's digit, two = 1's digit.
fn two_u8_to_u32(one: u8, two: u8) -> u32 {
    (one as u32 * 10) + two as u32
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

/// Add all invalid IDs together. Invalid IDs are sequences of digits that repeat
/// twice.
fn part1(file_name: &str) -> u32 {
    let file_contents = std::fs::read_to_string(file_name).expect("Couldn't open file");
    let banks = parse_text(&file_contents);
    let mut sum = 0;
    for mut bank in banks {
        sum += bank.find_largest_joltages();
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
        assert_eq!(part1("example1.txt"), 357);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1("input.txt"), 17311);
    }

    #[test]
    fn test_parse_text() {
        let input = "9876".to_string();
        let expected = vec![Bank::new(vec![9, 8, 7, 6])];
        assert_eq!(parse_text(&input), expected);
    }

    #[test]
    fn test_find_largest_joltage() {
        let mut bank = Bank::new(vec![9, 8, 7, 6]);
        assert_eq!(bank.find_largest_joltages(), 98);
        assert_eq!(bank.first, 9);
        assert_eq!(bank.first_pos, 0);
        assert_eq!(bank.second, 8);
        assert_eq!(bank.second_pos, 1);

        let mut bank = Bank::new(vec![8, 1, 1, 9]);
        assert_eq!(bank.find_largest_joltages(), 89);
        assert_eq!(bank.first, 8);
        assert_eq!(bank.first_pos, 0);
        assert_eq!(bank.second, 9);
        assert_eq!(bank.second_pos, 3);

        let mut bank = Bank::new(vec![2, 3, 4, 2, 7, 8]);
        assert_eq!(bank.find_largest_joltages(), 78);
        assert_eq!(bank.first, 7);
        assert_eq!(bank.first_pos, 4);
        assert_eq!(bank.second, 8);
        assert_eq!(bank.second_pos, 5);
    }
}
