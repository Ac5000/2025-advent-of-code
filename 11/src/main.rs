//! Day 11: Reactor

use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::Display,
};

/// Struct representing a device.
#[derive(Clone, Copy, Hash, Debug, Eq, PartialEq)]
struct Device([char; 3]);

impl TryFrom<&str> for Device {
    type Error = &'static str;

    /// Tries to convert &str to a Device. Panics if &str has wrong number of characters.
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() != 3 {
            Err("Incorrect character count to make a Device.")
        } else {
            // let tuple: (char, char, char) = value.chars().into();
            let mut array: [char; 3] = [' '; 3];
            for (index, character) in value.char_indices() {
                array[index] = character;
            }
            Ok(Self(array))
        }
    }
}

impl Display for Device {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}{}", self.0[0], self.0[1], self.0[2])
    }
}

/// Parse the incoming file to HashMap of Device keys with Devices values.
fn parse_text(string: &String) -> HashMap<Device, Vec<Device>> {
    // Make an empty HashMap with capacity equal to the number of lines/devices
    // plus one for the special "out" key.
    let mut devices = HashMap::with_capacity(string.lines().count() + 1);

    for line in string.lines() {
        // Split on the colon space to get two pieces of text.
        let mut string_parts: VecDeque<&str> = line.split(": ").collect();
        // First piece should be the key/node device. Convert to Device type.
        let key_device =
            Device::try_from(string_parts.pop_front().expect("parse_text->device.")).unwrap();

        // At this point, string_parts should only be the Devices that are connected
        // to the key_device. Pop off the VecDeque and split on the spaces to get
        // what should be a list of the Devices. Convert to Devices and push to
        // the Vec that will become the value for the entry.
        let mut connected_devices = Vec::new();
        for part in string_parts.pop_back().unwrap().split(' ') {
            connected_devices.push(Device::try_from(part).unwrap());
        }
        devices.insert(key_device, connected_devices);
    }

    // Insert the "out" Device.
    devices.insert(Device(['o', 'u', 't']), Vec::new());
    devices
}

/// Find number of paths from "you" to "out".
fn part1(file_name: &str) -> usize {
    let file_contents = std::fs::read_to_string(file_name).expect("Couldn't open file");
    let devices = parse_text(&file_contents);
    let mut ret = 0;
    // Keep track of devices that we already checked connections.
    let mut seen_devices = HashSet::new();
    // Queue of devices we still need to check/explore.
    let mut queue = VecDeque::new();
    // Seed the queue with our starting point.
    queue.push_back(&Device(['y', 'o', 'u']));

    // While loop keeps going until the queue is empty.
    while let Some(device) = queue.pop_front() {
        // Try to get the connections for the device or panic if we can't.
        if let Some(connections) = devices.get(&device) {
            // Loop the connections and add each to the queue if we haven't already
            // checked it.
            for connection in connections {
                if !seen_devices.contains(&connection) {
                    queue.push_back(connection);
                }
                // Count each time we see the "out" device for the path count.
                if connection == &Device(['o', 'u', 't']) {
                    ret += 1;
                }
            }
        } else {
            panic!("Can this happen?");
        };
        seen_devices.insert(device);
    }
    ret
}

/// Main function / code entry point.
fn main() {
    println!("Sum for example1: {}", part1("example1.txt"));
    println!("Sum for input: {}", part1("input.txt"));
    // println!("Sum for example1 part2: {}", part2("example1.txt"));
    // println!("Sum for input part2: {}", part2("input.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test against the example.
    #[test]
    fn part1_example01() {
        assert_eq!(part1("example1.txt"), 5);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1("input.txt"), 574);
    }

    // /// Test against the example.
    // #[test]
    // fn part2_example01() {
    //     assert_eq!(part2("example1.txt"), 24);
    // }
    //
    // #[test]
    // fn test_part2() {
    //     let part2 = part2("input.txt");
    //     assert!(part2 > 19851568); // First attempt too low. Also took > 10 minutes..
    //     assert_eq!(part2, 1603439684);
    // }

    #[test]
    fn test_convert_to_device() {
        assert_eq!(
            Device::try_from("aa"),
            Err("Incorrect character count to make a Device.")
        );
        assert_eq!(Device::try_from("aaa"), Ok(Device(['a', 'a', 'a'])));
    }
}
