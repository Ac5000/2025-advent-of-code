//! Day 01: Secret Entrance

/// Struct representing the dial.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Dial {
    /// Current number the dial is pointing to.
    number: i32,
    /// Count of times the dial pointed to zero.
    zero_count: u32,
    /// Count of times the dial clicked to zero.
    zero_count2: u32,
}

impl Dial {
    /// Make a new Dial from starting number.
    pub fn new(number: i32) -> Self {
        Self {
            number,
            zero_count: 0,
            zero_count2: 0,
        }
    }

    /// Add a Rotation to the Dial.
    fn add_rotation(&mut self, rotation: Rotation) {
        match rotation {
            Rotation::Left(i) => self.number = self.number - i as i32,
            Rotation::Right(i) => self.number = self.number + i as i32,
        };

        // Above 99, subtract 100's until below 100.
        while self.number > 99 {
            self.number = self.number - 100;
        }

        // Below zero, add 100's until positive.
        while self.number < 0 {
            self.number = self.number + 100;
        }

        // Add zero_count if we stopped at 0
        if self.number == 0 {
            self.zero_count = self.zero_count + 1;
        }
    }

    /// Add a Rotation to the Dial. Increment zero_count2 when crossing 0.
    fn add_rotation2(&mut self, rotation: Rotation) {
        println!(
            "Dial::add_rotation2: Dial starting at {}. zero_count2: {}. Rotation: {rotation:?}",
            self.number, self.zero_count2
        );

        // If we start at zero, we don't count first click in left direction.
        let mut starting_zero = self.number == 0;

        match rotation {
            Rotation::Left(i) => self.number = self.number - i as i32,
            Rotation::Right(i) => self.number = self.number + i as i32,
        };

        if self.number > 99 {
            // Above 99, subtract 100's until below 100.
            while self.number > 99 {
                self.number = self.number - 100;
                self.zero_count2 = self.zero_count2 + 1;
            }
        } else if self.number < 0 {
            // Below zero, add 100's until positive.
            while self.number < 0 {
                self.number = self.number + 100;
                if !starting_zero {
                    self.zero_count2 = self.zero_count2 + 1;
                } else {
                    starting_zero = false;
                }
            }
            if self.number == 0 {
                // Add zero_count if we stopped at 0
                self.zero_count2 = self.zero_count2 + 1;
            }
        } else if self.number == 0 {
            // Add zero_count if we stopped at 0 without going over.
            self.zero_count2 = self.zero_count2 + 1;
        }

        println!(
            "Dial::add_rotation AFTER: Dial at {}. zero_count2: {}.",
            self.number, self.zero_count2
        );
    }
}

/// Enum representing direction to spin the dial and distance/steps.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Rotation {
    /// Rotation "Left". Will subtract from current location.
    Left(i16),
    /// Rotation "Right". Will add to current location.
    Right(i16),
}

impl From<&str> for Rotation {
    /// Returns a rotation with step count from a string.
    fn from(value: &str) -> Self {
        let (direction, steps) = value.split_at(1);
        match direction {
            "L" => Self::Left(
                i16::from_str_radix(steps, 10)
                    .expect(&format!("Left failed to convert {steps} to steps.")),
            ),
            "R" => Self::Right(
                i16::from_str_radix(steps, 10)
                    .expect(&format!("Right failed to convert {steps} to steps.")),
            ),
            _ => panic!("Rotation::from_str got unexpected direction: {}", direction),
        }
    }
}

/// Parse the incoming text to Vec of Rotation's
fn parse_text(string: &String) -> Vec<Rotation> {
    let mut ret: Vec<Rotation> = Vec::new();
    for line in string.lines() {
        let rotation = Rotation::from(line);
        ret.push(rotation);
    }
    ret
}

/// Count the number of times the dial points to 0.
fn part1(file_name: &str) -> u32 {
    let file_contents = std::fs::read_to_string(file_name).expect("Couldn't open file");
    let rotations = parse_text(&file_contents);
    let mut dial = Dial::new(50);
    for rotation in rotations {
        dial.add_rotation(rotation);
    }
    dial.zero_count
}

/// Count the number of times the dial "clicks" to 0.
fn part2(file_name: &str) -> u32 {
    let file_contents = std::fs::read_to_string(file_name).expect("Couldn't open file");
    let rotations = parse_text(&file_contents);
    let mut dial = Dial::new(50);
    for rotation in rotations {
        dial.add_rotation2(rotation);
    }
    dial.zero_count2
}

/// Main function / code entry point.
fn main() {
    println!(
        "Number of times dial stopped at 0 for example1: {}",
        part1("example1.txt")
    );
    println!(
        "Number of times dial stopped at 0 for input: {}",
        part1("input.txt")
    );
    println!(
        "Number of times dial clicked 0 for example1: {}",
        part2("example1.txt")
    );
    println!(
        "Number of times dial clicked 0 for input: {}",
        part2("input.txt")
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test against the example.
    #[test]
    fn part1_example01() {
        assert_eq!(part1("example1.txt"), 3);
    }

    /// Test for part 1.
    #[test]
    fn test_part1() {
        assert_eq!(part1("input.txt"), 962);
    }

    /// Test parse_text.
    #[test]
    fn test_parse_text() {
        let input = "L1\nR2".to_string();
        let expected = vec![Rotation::Left(1), Rotation::Right(2)];
        assert_eq!(parse_text(&input), expected);
    }

    /// Test adding a Rotation to a Dial.
    #[test]
    fn test_add_rotation() {
        let mut dial = Dial::new(50);
        dial.add_rotation(Rotation::Right(5));
        assert_eq!(dial.number, 55);
        dial.add_rotation(Rotation::Right(44));
        assert_eq!(dial.number, 99);
        dial.add_rotation(Rotation::Right(1));
        assert_eq!(dial.number, 0);
        assert_eq!(dial.zero_count, 1);
        dial.add_rotation(Rotation::Left(1));
        assert_eq!(dial.number, 99);
        dial.add_rotation(Rotation::Left(99));
        assert_eq!(dial.number, 0);
        assert_eq!(dial.zero_count, 2);
    }

    /// Test adding Rotations to a Dial for part 2 using the example.
    /// My first attempt passed for overall number but failed getting correct answer.
    /// So I made this test for step by step checking against provided example.
    /// The example doesn't have you test against > 100 rotations which can cause
    /// an edge case I tested for separately below.
    #[test]
    fn test_add_rotation2() {
        let mut dial = Dial::new(50);
        assert_eq!(dial.number, 50);
        assert_eq!(dial.zero_count2, 0);

        dial.add_rotation2(Rotation::Left(68));
        assert_eq!(dial.number, 82);
        assert_eq!(dial.zero_count2, 1);

        dial.add_rotation2(Rotation::Left(30));
        assert_eq!(dial.number, 52);
        assert_eq!(dial.zero_count2, 1);

        dial.add_rotation2(Rotation::Right(48));
        assert_eq!(dial.number, 0);
        assert_eq!(dial.zero_count2, 2);

        dial.add_rotation2(Rotation::Left(5));
        assert_eq!(dial.number, 95);
        assert_eq!(dial.zero_count2, 2);

        dial.add_rotation2(Rotation::Right(60));
        assert_eq!(dial.number, 55);
        assert_eq!(dial.zero_count2, 3);

        dial.add_rotation2(Rotation::Left(55));
        assert_eq!(dial.number, 0);
        assert_eq!(dial.zero_count2, 4);

        dial.add_rotation2(Rotation::Left(1));
        assert_eq!(dial.number, 99);
        assert_eq!(dial.zero_count2, 4);

        dial.add_rotation2(Rotation::Left(99));
        assert_eq!(dial.number, 0);
        assert_eq!(dial.zero_count2, 5);

        dial.add_rotation2(Rotation::Right(14));
        assert_eq!(dial.number, 14);
        assert_eq!(dial.zero_count2, 5);

        dial.add_rotation2(Rotation::Left(82));
        assert_eq!(dial.number, 32);
        assert_eq!(dial.zero_count2, 6);
    }

    /// Test adding full rotation(s) starting at 0. Should add 1 to count.
    #[test]
    fn test_add_rotation3() {
        let mut dial = Dial::new(50);
        assert_eq!(dial.number, 50);
        assert_eq!(dial.zero_count2, 0);

        dial.add_rotation2(Rotation::Left(50));
        assert_eq!(dial.number, 0);
        assert_eq!(dial.zero_count2, 1);

        dial.add_rotation2(Rotation::Right(100));
        assert_eq!(dial.number, 0);
        assert_eq!(dial.zero_count2, 2);

        dial.add_rotation2(Rotation::Right(200));
        assert_eq!(dial.number, 0);
        assert_eq!(dial.zero_count2, 4);

        dial.add_rotation2(Rotation::Left(100));
        assert_eq!(dial.number, 0);
        assert_eq!(dial.zero_count2, 5);

        dial.add_rotation2(Rotation::Left(200));
        assert_eq!(dial.number, 0);
        assert_eq!(dial.zero_count2, 7);
    }

    /// Test adding partial left rotation(s) starting at 0. Should add 0 to count.
    #[test]
    fn test_add_rotation4() {
        let mut dial = Dial::new(0);
        assert_eq!(dial.number, 0);
        assert_eq!(dial.zero_count2, 0);

        // Shouldn't count since it was already at 0 and didn't click through.
        dial.add_rotation2(Rotation::Left(5));
        assert_eq!(dial.number, 95);
        assert_eq!(dial.zero_count2, 0);
    }

    /// Test against the example.
    #[test]
    fn part2_example01() {
        assert_eq!(part2("example1.txt"), 6);
    }

    /// Test for part 2.
    #[test]
    fn test_part2() {
        let answer = part2("input.txt");

        // My first result was too low.
        assert!(answer > 5765);

        // Second time I got it after adding tests for starting at 0.
        assert!(answer == 5782);
    }
}
