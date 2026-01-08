//! Day 11: Reactor

use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::{Debug, Display},
};

/// Struct representing a device.
#[derive(Clone, Copy, Hash, Eq, PartialEq)]
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

impl Debug for Device {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}{}", self.0[0], self.0[1], self.0[2])
    }
}

/// "you" Device that acts as our starting point for paths.
const YOU: Device = Device(['y', 'o', 'u']);
/// "out" Device that acts as our ending point for paths.
const OUT: Device = Device(['o', 'u', 't']);
/// "svr" Device that acts as our starting point for paths for part 2.
const SVR: Device = Device(['s', 'v', 'r']);
/// "dac" Device that must be visited in part 2.
const DAC: Device = Device(['d', 'a', 'c']);
/// "fft" Device that must be visited in part 2.
const FFT: Device = Device(['f', 'f', 't']);

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
            Device::try_from(string_parts.pop_front().expect("parse_text->key_device.")).unwrap();

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
    devices.insert(OUT, Vec::new());
    devices
}

/// Find number of paths from "you" to "out".
fn part1(file_name: &str) -> usize {
    let file_contents = std::fs::read_to_string(file_name).expect("Couldn't open file");
    let devices = parse_text(&file_contents);
    find_paths(YOU, OUT, &devices).len()
}

/// Find paths from start to end.
fn find_paths(
    start: Device,
    end: Device,
    devices: &HashMap<Device, Vec<Device>>,
) -> Vec<Vec<Device>> {
    // Keep track of devices that we already checked connections.
    let mut seen_devices = HashSet::new();
    // Queue of devices we still need to check/explore.
    let mut queue: VecDeque<(Device, Vec<Device>)> = VecDeque::new();
    // Seed the queue with our starting point.
    queue.push_back((start, Vec::new()));
    // Paths that lead from start to end.
    let mut paths: Vec<Vec<Device>> = Vec::new();

    // While loop keeps going until the queue is empty.
    while let Some((device, mut path)) = queue.pop_front() {
        seen_devices.insert(device);
        // Adds the starting point when necessary.
        if path.is_empty() {
            path.push(device);
        }
        // Try to get the connections for the device or panic if we can't.
        if let Some(connections) = devices.get(&device) {
            // Loop the connections and add each to the queue if we haven't already
            // checked it.
            for connection in connections {
                // Sanity check that the connection is in the device list.
                assert!(
                    devices.contains_key(connection),
                    "connection doesn't exist in the devices: {}",
                    connection
                );

                // Once we find end, add to the current path and push the
                // path into final paths. Continue loop so as not to affect queue.
                if connection == &end {
                    path.push(end);
                    paths.push(path.clone());
                    continue;
                }

                if !seen_devices.contains(&connection) {
                    let mut new_path = path.clone();
                    new_path.push(*connection);
                    queue.push_back((*connection, new_path));
                }
            }
        } else {
            panic!("Can this happen?");
        };
    }
    paths
}

/// Recursively counts the number of paths from `current` to `OUT` that contain
/// FFT and DAC.
/// `memo` is a memoization HashMap to help skip through stuff we saw already.
/// `map` is a HashMap of the input/example.
/// `current` is the Device this function is currently looking at.
/// `seen` is a pattern that we use to only count paths that have contain FFT and DAC.
/// I didn't fully come up with this on my own. For some reason my original attempts
/// weren't memoizing properly and would generate different values...
fn count_paths<'a>(
    memo: &mut HashMap<(&'a Device, [bool; 2]), u64>,
    map: &'a HashMap<Device, Vec<Device>>,
    current: &'a Device,
    seen @ [fft, dac]: [bool; 2],
) -> u64 {
    let entry = (current, seen);

    // Use memoization to skip paths we've already checked.
    if let Some(memo) = memo.get(&entry) {
        return *memo;
    }

    // If we find OUT, return with either 1 or 0. 1 being that path contains FFT
    // and DAC.
    if current == &OUT {
        return (fft && dac) as u64;
    }

    // Determine if we've seen FFT and DAC so far.
    let seen = [fft || current == &FFT, dac || current == &DAC];

    // Iterate through the connections for this device and recursively call this function.
    let out = (map[current].iter())
        .map(|child| count_paths(memo, map, child, seen))
        .sum();

    // Memoize our results to not repeat work.
    memo.insert(entry, out);
    out
}

/// Find number of paths from "svr" to "out" that include "dac" AND "fft".
fn part2_2(file_name: &str) -> u64 {
    let file_contents = std::fs::read_to_string(file_name).expect("Couldn't open file");
    let devices = parse_text(&file_contents);
    count_paths(&mut HashMap::new(), &devices, &SVR, [false, false])
}

/// Main function / code entry point.
fn main() {
    println!("Sum for example1: {}", part1("example1.txt"));
    println!("Sum for input: {}", part1("input.txt"));
    println!("Sum for example2 part2: {}", part2_2("example2.txt"));
    println!("Sum for input part2: {}", part2_2("input.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test against the example.
    #[test]
    fn part1_example1() {
        assert_eq!(part1("example1.txt"), 5);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1("input.txt"), 574);
    }

    /// Test against the example.
    #[test]
    fn part2_example2() {
        assert_eq!(part2_2("example2.txt"), 2);
    }

    #[test]
    fn test_part2() {
        let part2 = part2_2("input.txt");
        assert!(part2 > 36240, "{part2} > 36240"); // First attempt failed for too low.
        assert!(part2 > 289920, "{part2} > 289920"); // Second attempt failed for too low.
        assert_eq!(part2, 306594217920240);
    }

    #[test]
    fn test_convert_to_device() {
        assert_eq!(
            Device::try_from("aa"),
            Err("Incorrect character count to make a Device.")
        );
        assert_eq!(Device::try_from("aaa"), Ok(Device(['a', 'a', 'a'])));
    }
}
