//! Day 01: Secret Entrance

/// Struct representing the dial.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Dial {
    /// Current number the dial is pointing to.
    number: i32,
    /// Count of times the dial pointed to zero.
    zero_count: u32,
}

impl Dial {
    /// Make a new Dial from starting number.
    pub fn new(number: i32) -> Self {
        Self {
            number,
            zero_count: 0,
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
    println!("Number of times dial stopped at 0: {}", dial.zero_count);
    dial.zero_count
}

/// Main function / code entry point.
fn main() {
    part1("example1.txt");
    part1("input.txt");
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

    // /// Test against the example.
    // #[test]
    // fn part2_example01() {
    //     assert_eq!(part2("example.txt"), 30);
    // }
    //
    // /// Test for part 2.
    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2("input.txt"), 13114317);
    // }
}
