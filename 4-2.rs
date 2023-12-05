use std::io;
use std::collections::{HashMap, HashSet};

fn main() {
    let lines  = io::stdin().lines();
    let mut total = 0;
    let mut matches: HashMap<u32, u32> = HashMap::new();
    let mut process_stack: Vec<u32> = Vec::new();

    for maybe_line in lines {
        let line = maybe_line.unwrap();
        let mut split = line.split(": ");
        let card_info = split.next().unwrap();
        let (_, card_id) = card_info.split_once(" ").unwrap();
        let card_id = card_id.trim().parse::<u32>().unwrap();

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
        
        matches.insert(card_id, our_set.len() as u32);
        for won_id in (card_id + 1)..(card_id + 1 + our_set.len() as u32) {
            process_stack.push(won_id);
        }
        total += 1;
    }
    while !process_stack.is_empty() {
        let card_id = process_stack.pop().unwrap();
        let total_won = matches.get(&card_id).unwrap();
        for won_id in (card_id + 1)..(card_id + 1 + total_won) {
            process_stack.push(won_id);
        }
        total += 1;
    }
    println!("{}", total);
}
