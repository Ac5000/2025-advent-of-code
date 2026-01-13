//! Day 12: Christmas Tree Farm

use std::collections::HashMap;
use utilities::{Coord, coord};

#[derive(Clone, Copy, Debug, Default, Hash, Eq, PartialEq)]
struct Region {
    /// First number before 'x'
    width: usize,
    /// Second number after 'x'
    length: usize,
    /// Qty of shape 0
    shapes0: usize,
    /// Qty of shape 1
    shapes1: usize,
    /// Qty of shape 2
    shapes2: usize,
    /// Qty of shape 3
    shapes3: usize,
    /// Qty of shape 4
    shapes4: usize,
    /// Qty of shape 5
    shapes5: usize,
}

impl Region {
    /// Create a new Region. Panics if shapes.len() != 6.
    fn new(width: usize, length: usize, shapes: Vec<usize>) -> Self {
        assert_eq!(shapes.len(), 6, "Regions::new got wrong # of shapes.");

        Self {
            width,
            length,
            shapes0: shapes[0],
            shapes1: shapes[1],
            shapes2: shapes[2],
            shapes3: shapes[3],
            shapes4: shapes[4],
            shapes5: shapes[5],
        }
    }

    /// Compute and return the area of the Region.
    fn area(&self) -> usize {
        self.width * self.length
    }

    /// Returns the number of 3x3 rectangles that could fit in this region.
    fn rectangles(&self) -> usize {
        (self.width / 3) * (self.length / 3)
    }

    /// Returns the total quantity of shapes for this region.
    fn qty_shapes(&self) -> usize {
        self.shapes0 + self.shapes1 + self.shapes2 + self.shapes3 + self.shapes4 + self.shapes5
    }
}

impl TryFrom<&str> for Region {
    type Error = &'static str;

    /// Tries to convert &str to a Region. Panics if &str has wrong data.
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (area, shapes) = match value.split_once(": ") {
            None => return Err("Failed to split on ': '"),
            Some((area, shapes)) => (area, shapes),
        };
        let (width, length) = match area.split_once('x') {
            None => return Err("Failed to split area on 'x'"),
            Some((width, length)) => (width.parse::<usize>(), length.parse::<usize>()),
        };
        let width = match width {
            Ok(w) => w,
            Err(_) => return Err("Failed to parse width."),
        };
        let length = match length {
            Ok(l) => l,
            Err(_) => return Err("Failed to parse length."),
        };
        let shapes: Vec<_> = shapes
            .split(' ')
            .map(|x| x.parse::<usize>().expect("Failed to parse shape."))
            .collect();

        if shapes.len() != 6 {
            Err("Not enough shape values to make a Region.")
        } else {
            Ok(Self {
                width,
                length,
                shapes0: shapes[0],
                shapes1: shapes[1],
                shapes2: shapes[2],
                shapes3: shapes[3],
                shapes4: shapes[4],
                shapes5: shapes[5],
            })
        }
    }
}

/// Structure representing the various shapes/presents.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct Shape {
    /// Map of the shape where values of `true` are parts of the shape.
    map: HashMap<Coord, bool>,
}

impl Shape {
    /// Calculate and returns the area of the shape by counting `true` values from
    /// the map.
    fn area(&self) -> usize {
        self.map.iter().filter(|f| f.1 == &true).count()
    }
}

/// Parse the incoming file to HashMap of Device keys with Devices values.
fn parse_text(string: &String) -> (HashMap<u8, Shape>, Vec<Region>) {
    let mut regions = Vec::new();
    let mut shape_index = 0;
    let mut shape = Shape::default();
    let mut shapes: HashMap<u8, Shape> = HashMap::with_capacity(6);
    let mut shape_line_start = 0;

    for (line_num, line) in string.lines().enumerate() {
        // "Regions" contain 'x' and "Shapes" do not.
        if line.contains('x') {
            regions.push(Region::try_from(line).expect("Failed to get Region from line: {line}"));
        }
        // "Shape" indexes are just 2 characters. Update shape_index when we hit one.
        if line.len() == 2 {
            shape_index = line
                .strip_suffix(':')
                .expect("Failed to strip : from shape index")
                .parse()
                .expect("Failed to parse shape index");
            // Make a new shape in preparation to fill it.
            shape = Shape::default();
            // Set the shape_line_start to the next line number.
            shape_line_start = line_num + 1;
        }
        // Shapes have `#` or `.`.
        if line.contains('#') || line.contains('.') {
            // Shape y values are relative to top of the shape.
            let y = line_num - shape_line_start;
            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    shape.map.insert(coord!(x as i32, y as i32), true);
                } else if c == '.' {
                    shape.map.insert(coord!(x as i32, y as i32), false);
                } else {
                    panic!("Unrecognized character on line: {line}");
                }
            }
        }
        // Empty line, insert shape to HashMap if it's not default and clear it.
        if line.is_empty() {
            if shape != Shape::default() {
                shapes.insert(shape_index, shape.clone());
                shape = Shape::default();
            }
        }
    }

    assert_eq!(shapes.len(), 6, "parse_text didn't find correct # shapes.");

    (shapes, regions)
}

/// Simple check that the sum of the Shape areas in the region has to at least be
/// less than or equal to the Region area to fit.
/// true = shapes could fit. false = can not fit.
fn shapes_fit_in_area(region: &Region, shapes: &HashMap<u8, Shape>) -> bool {
    // Get all the shape areas. The complicated looking `shapes.get` is a way to
    // default to 0 if the shape doesn't exist in the map.
    let shapes0_area = region.shapes0 * shapes.get(&0).and_then(|s| Some(s.area())).unwrap_or(0);
    let shapes1_area = region.shapes1 * shapes.get(&1).and_then(|s| Some(s.area())).unwrap_or(0);
    let shapes2_area = region.shapes2 * shapes.get(&2).and_then(|s| Some(s.area())).unwrap_or(0);
    let shapes3_area = region.shapes3 * shapes.get(&3).and_then(|s| Some(s.area())).unwrap_or(0);
    let shapes4_area = region.shapes4 * shapes.get(&4).and_then(|s| Some(s.area())).unwrap_or(0);
    let shapes5_area = region.shapes5 * shapes.get(&5).and_then(|s| Some(s.area())).unwrap_or(0);

    // Sum the areas of the shapes to get a total coverage of the shapes.
    let total_shapes_area =
        shapes0_area + shapes1_area + shapes2_area + shapes3_area + shapes4_area + shapes5_area;

    total_shapes_area <= region.area()
}

/// Assume all shapes 3x3 rectangles and see if they can all fit in the region.
/// If yes, then the we don't need to check rotations, flips, and nesting.
/// Returns true if the region could hold rectangles. false needs checked
fn rectangle_check(region: &Region) -> bool {
    region.rectangles() >= region.qty_shapes()
}

/// Find number of Regions that can hold the quantities of shapes.
fn part1(file_name: &str) -> usize {
    let file_contents = std::fs::read_to_string(file_name).expect("Couldn't open file");
    let (shapes, regions) = parse_text(&file_contents);
    println!("regions.len: {}", regions.len());
    let areas_filtered: Vec<_> = regions
        .iter()
        .filter(|&r| shapes_fit_in_area(r, &shapes))
        .collect();
    let areas_filtered_len = areas_filtered.len();
    println!("areas_filtered.len: {}", areas_filtered_len);

    let rectangles_filtered: Vec<_> = areas_filtered
        .iter()
        .filter(|&r| !rectangle_check(r))
        .collect();
    // My brain broke the first time this ran and it returned 0. Thought I had an
    // error with my code. But no, after the area filter, all the other shapes
    // could easily fit.
    println!("rectangles_filtered.len: {}", rectangles_filtered.len());

    areas_filtered_len
}

/// Main function / code entry point.
fn main() {
    println!("Sum for example1: {}", part1("example1.txt"));
    println!("Sum for input: {}", part1("input.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test against the example.
    #[test]
    fn part1_example1() {
        assert_eq!(part1("example1.txt"), 2);
    }

    #[test]
    fn test_part1() {
        let part1 = part1("input.txt");
        assert!(part1 < 1000); // Logic problem cooked me on this one...
        assert_eq!(part1, 528);
    }

    #[test]
    #[should_panic(
        expected = "assertion `left == right` failed: Regions::new got wrong # of shapes.\n  left: 5\n right: 6"
    )]
    fn test_region_new_bad_vec() {
        let _ = Region::new(5, 5, vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn test_region_area() {
        let region = Region::new(5, 5, vec![0, 1, 2, 3, 4, 5]);
        assert_eq!(region.area(), 25);
    }

    #[test]
    fn test_region_rectangles() {
        assert_eq!(Region::new(0, 1, vec![1; 6]).rectangles(), 0);
        assert_eq!(Region::new(1, 0, vec![1; 6]).rectangles(), 0);
        assert_eq!(Region::new(3, 1, vec![1; 6]).rectangles(), 0);
        assert_eq!(Region::new(1, 3, vec![1; 6]).rectangles(), 0);
        assert_eq!(Region::new(3, 3, vec![1; 6]).rectangles(), 1);
        assert_eq!(Region::new(3, 6, vec![1; 6]).rectangles(), 2);
    }

    #[test]
    fn test_region_qty_shapes() {
        assert_eq!(Region::new(0, 1, vec![1; 6]).qty_shapes(), 6);
        assert_eq!(Region::new(0, 1, vec![0; 6]).qty_shapes(), 0);
        assert_eq!(Region::new(0, 1, vec![0, 1, 2, 3, 4, 5]).qty_shapes(), 15);
    }

    #[test]
    fn test_shape_area() {
        let mut shape = Shape::default();
        assert_eq!(shape.area(), 0);
        shape.map.insert(coord!(0, 0), true);
        assert_eq!(shape.area(), 1);
        shape.map.insert(coord!(0, 1), true);
        assert_eq!(shape.area(), 2);
        shape.map.insert(coord!(1, 1), true);
        assert_eq!(shape.area(), 3);
    }

    #[test]
    fn test_shapes_fit_in_area() {
        let region = Region::new(3, 3, vec![1, 0, 0, 0, 0, 0]);
        let mut shape = Shape::default();
        shape.map.insert(coord!(0, 0), true);
        shape.map.insert(coord!(0, 1), true);
        shape.map.insert(coord!(0, 2), true);
        assert_eq!(shape.area(), 3);
        let shape_area = shape.area();
        let mut shapes = HashMap::new();
        shapes.insert(0, shape);
        // Single shape in large enough space.
        assert_eq!(
            shapes_fit_in_area(&region, &shapes),
            true,
            "Shape area: {} should fit region area {}.",
            shape_area,
            region.area()
        );

        // Many of shape that won't fit.
        let region = Region::new(3, 3, vec![10, 0, 0, 0, 0, 0]);
        assert_eq!(
            shapes_fit_in_area(&region, &shapes),
            false,
            "10 Shape areas: {} should not fit region area {}.",
            shape_area,
            region.area()
        );
    }
    #[test]
    fn test_rectangle_check() {
        assert!(
            !rectangle_check(&Region::new(1, 1, vec![1, 0, 0, 0, 0, 0])),
            "no rects should fit in 1x1"
        );
        assert!(
            rectangle_check(&Region::new(3, 3, vec![1, 0, 0, 0, 0, 0])),
            "1 rect should fit in 3x3"
        );
        assert!(
            !rectangle_check(&Region::new(3, 3, vec![2, 0, 0, 0, 0, 0])),
            "2 rects should not fit in 3x3"
        );
        assert!(
            !rectangle_check(&Region::new(3, 3, vec![0, 0, 1, 0, 1, 0])),
            "2 rects should not fit in 3x3"
        );
        assert!(
            !rectangle_check(&Region::new(5, 5, vec![0, 0, 1, 0, 1, 0])),
            "2 rects should not fit in 5x5"
        );
        assert!(
            rectangle_check(&Region::new(6, 3, vec![0, 0, 1, 0, 1, 0])),
            "2 rects should in 6x3"
        );
    }
}
