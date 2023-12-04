use std::io;
use std::collections::{HashMap, HashSet};

fn main() {
    let lines = io::stdin().lines();

    /* Keep track of the coordinates for each part number and each symbol.
     * The part coordinates will have the last column of the part number.
     */
    let mut part_coordinates: HashMap<(i32, i32), u32> = HashMap::new();
    let mut symbol_coordinates: HashSet<(i32, i32)> = HashSet::new();

    for (row, maybe_line) in lines.enumerate() {
        let mut current_number: Option<u32> = None;

        let line = maybe_line.unwrap();
        for (col, c) in line.chars().enumerate() {
            if c.is_ascii_digit() {
                /* Keep accumulating digits */
                current_number = match current_number {
                    Some(e) => Some(e * 10 + c.to_digit(10).unwrap()),
                    None => Some(c.to_digit(10).unwrap())
                }
            } else {
                if c != '.' {
                    symbol_coordinates.insert((col as i32, row as i32));
                }
                /* If we have a number, store its coordinates */
                match current_number {
                    Some(e) => {
                        /* Previous column was the end column for the number,
                         * so be sure to subtract 1 from the current column.
                         */
                        part_coordinates.insert((col as i32 - 1, row as i32), e);
                        current_number = None;
                    },
                    None => ()
                }
            }
        }
        /* Hande edge case where a number is at the end of a line. */
        match current_number {
            Some(e) => {
                part_coordinates.insert((line.len() as i32 - 1, row as i32), e);
            },
            None => ()
        }
    }

    let mut total = 0;

    /* Starting at the end column for each part number, check all neighboring
     * coordinates for a symbol, moving to the left for the length of the number.
     */
    for ((end_col, row), number) in part_coordinates {
        let mut part_found = false;
        let mut order = 0;
        while number >= 10_u32.pow(order) {
            for col_offset in -1..=1 {
                for row_offset in -1..=1 {
                    let coordinate = (end_col + col_offset - order as i32, row + row_offset);
                    if symbol_coordinates.contains(&coordinate) {
                        part_found = true;
                    }
                }
            }
            order += 1;
        }
        if part_found {
            total += number;
        }
    }
    println!("{}", total);
}
