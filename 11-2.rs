use std::io;
use std::iter::FromIterator;

/* Return a Vec of pairs of galaxies, having all combinations
 * of galaxies returned.
 */
pub fn combinations(galaxies: &Vec<(i128, i128)>) -> Vec<((i128, i128), (i128, i128))>
{
	let r = 2;
    let pool: Vec<(i128, i128)> = galaxies.iter().cloned().collect();
    let mut results = Vec::new();
    let n = pool.len();
    if n < r {
        return results;
    }
    let mut indices = Vec::from_iter(0..r);
    results.push((pool[0], pool[1]));
    loop {
        let idx = (0..r).rev().find(|i| indices[*i] != i + n - r);
        match idx {
            Some(i) => {
                indices[i] += 1;
                for j in (i + 1)..r {
                    indices[j] = indices[j - 1] + 1;
                }
                results.push((pool[indices[0]], pool[indices[1]]));
            }
            None => break,
        }
    }
    results
}

fn main() {
    let lines = io::stdin().lines();
    let mut row_galaxy_count: Vec<u32> = Vec::new();
    let mut col_galaxy_count: Vec<u32> = Vec::new();
    let mut galaxies: Vec<(i128, i128)> = Vec::new();
    /* Parse the galaxies. Keep track of each galaxy as well as
     * the galaxy count for each row and column.
     */
    for (row, maybe_line) in lines.enumerate() {
        let line = maybe_line.unwrap();
        let mut row_count = 0;
        while col_galaxy_count.len() < line.len() {
            col_galaxy_count.push(0);
        }
        for (idx, c) in line.chars().enumerate() {
            if c == '#' {
                galaxies.push((idx as i128, row as i128));
                row_count += 1;
                if let Some(col_count) = col_galaxy_count.get_mut(idx) {
                    *col_count += 1;
                }
            }
        }
        row_galaxy_count.push(row_count);
    }
    let mut current_row = 0;
    /* For each row that has zero galaxies, move every galaxy down
     * by 999,999 that has a row number greater than the current row.
     * Also, increase the current row by 999,999 to account for the expansion.
     */
    row_galaxy_count
        .iter()
        .for_each(|row_count| {
            if *row_count == 0 {
                for galaxy in galaxies
                    .iter_mut()
                    .filter(|x| x.1 > current_row) {
                        *galaxy = (galaxy.0, galaxy.1 + 999_999);
                }
                current_row += 999_999;
            }
            current_row += 1;
        });
    let mut current_col = 0;
    /* For each col that has zero galaxies, move every galaxy down
     * by 999,999 that has a col number greater than the current col.
     * Also, increase the current col by 999,999 to account for the expansion.
     */
    col_galaxy_count
        .iter()
        .for_each(|col_count| {
            if *col_count == 0 {
                for galaxy in galaxies
                    .iter_mut()
                    .filter(|x| x.0 > current_col) {
                        *galaxy = (galaxy.0 + 999_999, galaxy.1);
                    }
                current_col += 999_999;
            }
            current_col += 1;
        });
    let total = combinations(&galaxies)
        .iter()
        .fold(0, |acc, (left, right)| {
            /* Just compute the Manhattan distance for each pair */
            acc + (right.0 - left.0).abs() + (right.1 - left.1).abs()
        });
    println!("{}", total);
}
