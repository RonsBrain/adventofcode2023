use std::io;
use std::collections::HashSet;

fn main() {
    let lines  = io::stdin().lines();
    let mut total = 0;

    for maybe_line in lines {
        let line = maybe_line.unwrap();
        let mut split = line.split(": ");
        /* Ignore the card number */
        split.next();

        /* Parse the number sections */
        let numbers = split.next().unwrap();
        let mut split = numbers.split(" | ");
        let winning_numbers = split.next().unwrap();
        let our_numbers = split.next().unwrap();

        /* Parse and collect the winning numbers */
        let winning_set: HashSet<u32> = winning_numbers.split(" ")
            .filter(|x| *x != "")
            .map(|x| x.parse::<u32>().unwrap())
            .collect();

        /* Parse and collect our numbers, discardign the ones
         * that are not winning numbers.
         */
        let our_set: HashSet<u32> = our_numbers.split(" ")
            .filter(|x| *x != "")
            .map(|x| x.parse::<u32>().unwrap())
            .filter(|x| winning_set.contains(x))
            .collect();

        /* If we won, calculate the points and add it to the total. */
        if our_set.len() > 0 {
            total += 2_u32.pow(our_set.len() as u32 - 1);
        }
    }
    println!("{}", total);
}
