use std::io;

fn main() {
    let lines = io::stdin().lines();
    let mut total: i64 = 0;

    for maybe_line in lines {
        let line = maybe_line.unwrap();
        /* Parse the numbers in the line */
        let mut numbers: Vec<i64> = line
            .split(" ")
            .map(|x| x.parse::<i64>().unwrap())
            .collect();
        /* Keep track of the last number as each sequence is computed */
        let mut pattern_end: Vec<i64> = Vec::new();
        while !numbers.iter().cloned().all(|x| x == 0) {
            /* Store the last number in this sequence */
            pattern_end.push(*numbers.last().unwrap());
            /* Compute the next sequence */
            numbers = numbers
                .as_slice()
                .windows(2)
                .map(|x| x[1] - x[0])
                .collect();
        }
        /* Compute the extrapolated values for each sequence and add the
         * sum of them to the total */
        total += pattern_end
            .iter()
            .fold(0, |acc, x| acc + x);
    }
    println!("{}", total);
}
