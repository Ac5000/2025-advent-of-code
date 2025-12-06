//! Day 06: Trash Compactor

use std::collections::HashMap;

/// Enum that represents the operation to take place on a column.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Operation {
    Add,
    Multiply,
    NoOp,
}

impl From<&str> for Operation {
    /// Returns an operation type from a string.
    fn from(value: &str) -> Self {
        match value {
            "+" => Self::Add,
            "*" => Self::Multiply,
            _ => panic!("Operation::from_str got unexpected: {}", value),
        }
    }
}
impl Default for Operation {
    /// Returns the default of no operation (NoOp)
    fn default() -> Self {
        Self::NoOp
    }
}

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
struct Column {
    numbers: Vec<u64>,
    operation: Operation,
}

impl Column {
    /// Make a new Column from Vec(row, &str).
    fn new_from_raw(mut entry: Vec<&str>) -> Self {
        let mut ret = Self::default();
        ret.operation = Operation::from(entry.pop().expect("Failed to pop Operation."));
        for number in entry {
            ret.numbers.push(
                number
                    .parse()
                    .expect(&format!("Failed to parse: {}", number)),
            );
        }
        ret
    }
}

/// Get result of the Column operation.
fn get_result(column: Column) -> u64 {
    match column.operation {
        Operation::Add => column.numbers.into_iter().sum(),
        Operation::Multiply => column.numbers.into_iter().product(),
        _ => panic!("get_result has no operation."),
    }
}

/// Parse the incoming file to HashMap of column#,Column.
fn parse_text(string: &String) -> HashMap<usize, Column> {
    let mut columns_raw: HashMap<usize, Vec<&str>> = HashMap::new();
    let mut columns: HashMap<usize, Column> = HashMap::new();

    for line in string.lines() {
        for (column, item) in line.split_whitespace().enumerate() {
            columns_raw
                .entry(column)
                .and_modify(|x| x.push(item))
                .or_insert_with(|| vec![item]);
        }
    }

    for (column, items) in columns_raw {
        columns.insert(column, Column::new_from_raw(items));
    }
    columns
}

/// For each column of numbers, perform the operation at the bottom. Sum all results.
fn part1(file_name: &str) -> u64 {
    let file_contents = std::fs::read_to_string(file_name).expect("Couldn't open file");
    let columns = parse_text(&file_contents);
    let answer = columns.into_iter().map(|x| get_result(x.1)).sum();
    answer
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
        assert_eq!(part1("example1.txt"), 4277556);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1("input.txt"), 4583860641327);
    }

    // /// Test against the example.
    // #[test]
    // fn part2_example01() {
    //     assert_eq!(part2("example1.txt"), 14);
    // }
    //
    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2("input.txt"), 338258295736104);
    // }
}
