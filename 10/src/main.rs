//! Day 10: Factory

use std::{
    collections::{HashSet, VecDeque},
    fmt::Display,
};

/// Struct representing a machine.
#[derive(Clone, Hash, Debug, Eq, PartialEq)]
struct Machine {
    /// Use a u16 as a binary representation of the lights.
    lights_required: u16,
    /// Use a u16 as a binary representation of the lights.
    lights_state: u16,
    /// Use a Vec<u16> so each button's binary aligns with the lights.
    buttons: Vec<u16>,
    /// Use array of 16 u8's since it aligns with lights bits
    joltages: [u16; 16],
}

impl Machine {
    /// Create a new machine in default state.
    fn new(lights_required: u16, buttons: Vec<u16>, joltages: [u16; 16]) -> Self {
        Self {
            lights_required,
            lights_state: 0,
            buttons,
            joltages,
        }
    }

    /// Press a button to change the machine state.
    fn press_button(&mut self, button: u16) {
        self.lights_state ^= button
    }
}

impl Display for Machine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Machine:\n\tlights_required: {:b}\n\tlights_state: {:b}",
            self.lights_required, self.lights_state
        )?;
        for button in &self.buttons {
            write!(f, "\n\tbutton: {:b}", button)?;
        }
        write!(f, "\n\tjoltages: {:?}", self.joltages)?;
        Ok(())
    }
}

/// Convert the lights string into a u16 via binary representation.
fn convert_lights(lights: &str) -> u16 {
    let lights = lights.replace('.', "0");
    let lights = lights.replace('#', "1");
    // Reverse the order here so it's easier to make buttons later.
    let lights_rev: String = lights.chars().rev().collect();
    u16::from_str_radix(&lights_rev, 2).expect("convert_lights failed.")
}

/// Convert the joltages string into an array of u16;16. Align the first digit with
/// the last index of the array so it aligns with the bits of the lights.
fn convert_joltages(joltages: &str) -> [u16; 16] {
    let mut ret: [u16; 16] = [0; 16];
    let splits: Vec<u16> = joltages
        .split(',')
        .map(|x| {
            x.parse::<u16>()
                .expect(&format!("convert_joltages failed for {x}"))
        })
        .collect();
    if splits.len() > 16 {
        panic!("too many joltages.");
    }
    for (idx, num) in splits.into_iter().enumerate() {
        ret[ret.len() - 1 - idx] = num;
    }
    ret
}

/// Convert the button schematic string into a u16. Align in reverse index order
/// so it aligns with the bits of the lights.
fn convert_button(button: &str) -> u16 {
    let mut bit_array: [bool; 16] = [false; 16];
    button
        .split(',')
        .map(|x| {
            x.parse::<usize>()
                .expect(&format!("convert_button failed for {x}"))
        })
        .for_each(|x| bit_array[bit_array.len() - 1 - x] = true);
    let mut value: u16 = 0;
    for (i, b) in bit_array.iter().rev().enumerate() {
        let bit = if *b { 1 } else { 0 };
        value |= bit << i;
    }

    value
}

/// Parse the incoming file to Vec of Coords.
fn parse_text(string: &String) -> Vec<Machine> {
    let mut machines = Vec::new();
    let x: &[_] = &['(', ')', '{', '}', '[', ']'];

    for line in string.lines() {
        let mut string_parts: VecDeque<&str> = line.split(' ').collect();
        let lights = string_parts
            .pop_front()
            .expect("parse_text->lights.")
            .trim_matches(x);
        let lights_converted = convert_lights(lights);
        let joltages = string_parts
            .pop_back()
            .expect("parse_text->joltages.")
            .trim_matches(x);
        let joltages_converted = convert_joltages(joltages);
        // At this point, string_parts should only be the buttons.
        let mut buttons = Vec::new();
        for part in string_parts {
            let button = part.trim_matches(x);
            buttons.push(convert_button(button));
        }
        machines.push(Machine::new(lights_converted, buttons, joltages_converted));
    }
    machines
}

/// For each machine, figure out minimum number of button presses to make indicator
/// lights match the diagram. Sum all the minimums for final answer.
fn part1(file_name: &str) -> u64 {
    let file_contents = std::fs::read_to_string(file_name).expect("Couldn't open file");
    let machines = parse_text(&file_contents);
    let mut ret = 0;

    for machine in machines {
        // queue is a tuple<T> where T.0 = the state of the lights and T.1 = number
        // of button presses it took to get to that state.
        let mut queue = VecDeque::new();
        // seen is a HashSet of all the states we've seen for the lights.
        let mut seen = HashSet::new();
        // Start off the queue with all lights off and no button presses.
        queue.push_back((0, 0));

        while let Some((lights, presses)) = queue.pop_front() {
            if lights == machine.lights_required {
                ret += presses;
                break;
            }

            for button in &machine.buttons {
                let next = lights ^ button;

                if !seen.contains(&next) {
                    seen.insert(next);
                    queue.push_back((next, presses + 1));
                }
            }
        }
    }
    ret
}

/// For each machine, figure out minimum number of button presses to make joltage
/// levels match the given diagram. Sum all the minimums for final answer.
fn part2(file_name: &str) -> u64 {
    let file_contents = std::fs::read_to_string(file_name).expect("Couldn't open file");
    let machines = parse_text(&file_contents);
    let mut ret = 0;

    for machine in machines {
        // queue is a tuple<T> where T.0 = the state of the lights and T.1 = number
        // of button presses it took to get to that state.
        let mut queue = VecDeque::new();
        // seen is a HashSet of all the states we've seen for the lights.
        let mut seen = HashSet::new();
        // Start off the queue with all lights off and no button presses.
        queue.push_back((0, 0));

        while let Some((lights, presses)) = queue.pop_front() {
            if lights == machine.lights_required {
                ret += presses;
                break;
            }

            for button in &machine.buttons {
                let next = lights ^ button;

                if !seen.contains(&next) {
                    seen.insert(next);
                    queue.push_back((next, presses + 1));
                }
            }
        }
    }
    ret
}

/// Main function / code entry point.
fn main() {
    println!("Sum for example1: {}", part1("example1.txt"));
    println!("Sum for input: {}", part1("input.txt"));
    println!("Sum for example1 part2: {}", part2("example1.txt"));
    // println!("Sum for input part2: {}", part2("input.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test against the example.
    #[test]
    fn part1_example01() {
        assert_eq!(part1("example1.txt"), 7);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1("input.txt"), 475);
    }

    /// Test against the example.
    #[test]
    fn part2_example01() {
        assert_eq!(part2("example1.txt"), 33);
    }

    // #[test]
    // fn test_part2() {
    //     let part2 = part2("input.txt");
    //     assert!(part2 > 19851568); // First attempt too low. Also took > 10 minutes..
    //     assert_eq!(part2, 1603439684);
    // }

    #[test]
    #[should_panic(expected = "convert_lights failed.")]
    fn test_convert_lights_fail() {
        convert_lights("FAIL");
    }

    #[test]
    fn test_convert_lights() {
        assert_eq!(convert_lights("0"), 0);
        assert_eq!(convert_lights("1"), 1);
        assert_eq!(convert_lights("01"), 2);
        assert_eq!(convert_lights("10"), 1);
        assert_eq!(convert_lights(".##."), 0b0110u16);
        assert_eq!(convert_lights("...#."), 0b01000u16);
    }

    #[test]
    fn test_convert_joltages() {
        assert_eq!(
            convert_joltages("3,5,4,7"),
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 4, 5, 3]
        );
        assert_eq!(
            convert_joltages("10,11,11,5,10,5"),
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 10, 5, 11, 11, 10]
        );
    }

    #[test]
    fn test_convert_button() {
        assert_eq!(convert_button("0,1"), 0b11u16);
        assert_eq!(convert_button("0,2"), 0b101u16);
        assert_eq!(convert_button("0,1,2"), 0b111u16);
    }

    #[test]
    fn test_press_button() {
        let mut machine = Machine::new(11, vec![], [0; 16]);
        assert_eq!(machine.lights_state, 0b0u16);
        machine.press_button(0b0u16);
        assert_eq!(machine.lights_state, 0b0u16);
        machine.press_button(0b1u16);
        assert_eq!(machine.lights_state, 0b1u16);
        machine.press_button(0b1u16);
        assert_eq!(machine.lights_state, 0b0u16);
        machine.press_button(0b10u16);
        assert_eq!(machine.lights_state, 0b10u16);
    }
}
