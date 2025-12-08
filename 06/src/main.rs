//! Day 06: Trash Compactor

use std::collections::HashMap;
use utilities::{coord::Coord, grid::Grid};

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
    /// Make a new Column from Vec(&str).
    fn new_from_raw(entry: Vec<&str>) -> Self {
        let mut ret = Self::default();
        for number in entry {
            if number == "+" || number == "*" {
                ret.operation = Operation::from(number);
                continue;
            };
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

/// Parse the incoming file to HashMap of column#,Column.
fn parse_text2(string: &String) -> HashMap<usize, Column> {
    let mut rows_raw: HashMap<usize, Vec<&str>> = HashMap::new();
    let mut rows: HashMap<usize, Column> = HashMap::new();
    let mut column_count: usize = 0;

    for line in string.lines() {
        // Increment the "column" count each empty line.
        if line.split_whitespace().count() == 0 {
            column_count += 1;
            continue;
        }

        for item in line.split_whitespace() {
            rows_raw
                .entry(column_count)
                .and_modify(|x| x.push(item))
                .or_insert_with(|| vec![item]);
        }
    }

    for (row, items) in rows_raw {
        rows.insert(row, Column::new_from_raw(items));
    }
    rows
}

/// For each column of numbers, perform the operation at the bottom. Sum all results.
fn part1(file_name: &str) -> u64 {
    let file_contents = std::fs::read_to_string(file_name).expect("Couldn't open file");
    let columns = parse_text(&file_contents);
    let answer = columns.into_iter().map(|x| get_result(x.1)).sum();
    answer
}

/// Rotate a grid counter-clockwise.
/// So this doesn't really rotate, it sorta flips the 2d matrix, but it works for
/// what I need...
fn rotate_grid_ccw(grid: &Grid) -> Grid {
    let mut new_grid = Grid::new();
    for (&coord, &char_val) in grid.char_map.iter() {
        let new_x = coord.y;
        let new_y = coord.x;

        let new_coord = Coord::new(new_x, new_y);
        new_grid.char_map.insert(new_coord, char_val);
    }
    new_grid.max_x = grid.max_y;
    new_grid.max_y = grid.max_x;

    new_grid
}

/// Convert the grid back to a string so we can use it in the logic that worked
/// for part1.
fn grid_to_string(grid: &Grid) -> String {
    let mut ret = String::new();
    for y in 0..=grid.max_y {
        for x in 0..=grid.max_x {
            let character = *grid
                .char_map
                .get(&Coord::new(x, y))
                .expect(&format!("Failed to find char at: {:?}", (x, y)));
            // Need to add a space before the operators so the parse by split_whitespace
            // works.
            if character == '+' || character == '*' {
                ret.push(' ');
                ret.push(character);
                ret.push(' ');
            } else {
                ret.push(character);
            };
        }
        // Push newline for each y/row.
        ret.push_str("\n");
    }

    ret
}

/// For each column of numbers, perform the operation at the bottom. Sum all results.
/// Read numbers right-to-left in each column.
fn part2(file_name: &str) -> u64 {
    let grid = Grid::new_from_file(file_name);
    let new_grid = rotate_grid_ccw(&grid);
    let new_string = grid_to_string(&new_grid);
    let columns = parse_text2(&new_string);
    let answer = columns.into_iter().map(|x| get_result(x.1)).sum();
    answer
}

/// Main function / code entry point.
fn main() {
    println!("Sum for example1: {}", part1("example1.txt"));
    println!("Sum for input: {}", part1("input.txt"));
    println!("Sum for example1 part2: {}", part2("example1.txt"));
    println!("Sum for input part2: {}", part2("input.txt"));
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

    /// Test against the example.
    #[test]
    fn part2_example01() {
        assert_eq!(part2("example1.txt"), 3263827);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("input.txt"), 11602774058280);
    }
}
