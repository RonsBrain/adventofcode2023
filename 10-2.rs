use std::io;
use std::collections::{HashMap, VecDeque};

#[derive(Debug, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

/* Find the neighbor coordinate for a direction. Does not allow for
 * an out-of-bounds cooridnate.
 */
fn neighbor(
    current: (usize, usize),
    direction: &Direction,
    bounds: (usize, usize),
) -> Option<(usize, usize)> {
    use Direction::*;
    let row_offset = match direction {
        Up => -1,
        Down => 1,
        _ => 0,
    };
    let col_offset = match direction {
        Left => -1,
        Right => 1,
        _ => 0,
    };
    if let (Some(col), Some(row)) = (current.0.checked_add_signed(col_offset), current.1.checked_add_signed(row_offset)) {
        if col == bounds.0 || row == bounds.1 {
            return None
        }
        return Some((col, row));
    }
    None
}

/* Find the next coordinate based on a symbol and direction */
fn next(
    current: (usize, usize),
    last_direction: &Direction,
    symbol: char,
    bounds: (usize, usize)
) -> Option<((usize, usize), Direction)> {
    use Direction::*;
    let next_direction = match symbol {
        '|' | '-' => *last_direction,
        'J' => match last_direction {
            Right => Up,
            Down => Left,
            _ => panic!("Came from {:?}", last_direction),
        },
        'L' => match last_direction {
            Left => Up,
            Down => Right,
            _ => panic!("Came from {:?}", last_direction),
        },
        'F' => match last_direction {
            Up => Right,
            Left => Down,
            _ => panic!("Came from {:?}", last_direction),
        },
        '7' => match last_direction {
            Right => Down,
            Up => Left,
            _ => panic!("Came from {:?}", last_direction),
        },
        _ => panic!("Illegal symbol {}", symbol),
    };
    if let Some(next_position) = neighbor(current, &next_direction, bounds) {
        return Some((next_position, next_direction));
    }
    None
}

/* Determine if a symbol will allow you to move on it in
 * a particular direction.
 */
fn can_move_to(symbol: char, direction: &Direction) -> bool {
    use Direction::*;
    match direction {
        Up => symbol == '|' || symbol == '7' || symbol == 'F',
        Down => symbol == '|' || symbol == 'J' || symbol == 'L',
        Left => symbol == '-' || symbol == 'L' || symbol == 'F',
        Right => symbol == '-' || symbol == 'J' || symbol == '7',
    }
}

fn main() {
    use Direction::*;
    let lines = io::stdin().lines();
    let mut start: (usize, usize) = (0, 0);
    let mut to_visit: VecDeque<((usize, usize), u32, Direction)> = VecDeque::new();
    let mut visited: HashMap<(usize, usize), (u32, Direction)> = HashMap::new();

    /* Parse the board */
    let board: Vec<Vec<char>> = lines
        .enumerate()
        .map(|(row, line)| {
            let result: Vec<char> = line.unwrap().chars().collect();
            if let Some(col) = result.iter().position(|c| *c == 'S') {
                start = (col, row);
            }
            result
        })
        .collect();

    let bounds = (board[0].len(), board.len());
    /* Find all the valid ways to move from the start location and push
     * them into the queue to visit.
     */
    for direction in [Up, Down, Left, Right] {
        if let Some((col, row)) = neighbor(start, &direction, bounds) {
            if can_move_to(board[row][col], &direction) {
                to_visit.push_back(((col, row), 1_u32, direction));
            }
        }
    }

    /* Do breadth first search, traversing the possible loop until we
     * find a coordinate we've already visited. This is the loop.
     */
    while !to_visit.is_empty() {
        let (current, steps, direction) = to_visit.pop_front().unwrap();
        visited.insert(current, (steps, direction));
        let symbol = board[current.1][current.0];
        if let Some((next_position, next_direction)) = next(current, &direction, symbol, bounds) {
            /* We can visit the next board position in this direction. Check to see
             * if we've discovered the loop.
             */
            if visited.contains_key(&next_position) {
                break;
            }

            /* We want to eventually visit the next position. */
            to_visit.push_back((next_position, steps + 1, next_direction));
        }
    }

    let mut inside_count = 0;
    for (row_num, row) in board.iter().enumerate() {
        /* Use a sort of point in polygon algorithm to find tiles
         * inside the loop.
         */
        let mut inside = false;
        for (col_num, symbol) in row.iter().enumerate() {
            if visited.contains_key(&(col_num, row_num)) {
                /* We hit a tile in the loop. */
                match symbol {
                    'L' | 'J' | '|' | 'S' => inside = !inside, // Tile goes up, toggle flag
                    _ => ()
                };
            }
            if inside && !visited.contains_key(&(col_num, row_num)) {
                /* We think we're inside the loop. Increase the count by 1 unless
                 * it's the start position.
                 */
                if (col_num, row_num) != start {
                    inside_count += 1;
                }
            }
        }
    }
    println!("{}", inside_count);
}
