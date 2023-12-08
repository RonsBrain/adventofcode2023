use std::io;
use std::cmp::Ordering;
use std::iter::zip;

#[derive(Debug)]
/* Struct to keep track of each hand */
struct Hand {
    pub cards: String,
    pub bid: u32,
    pub rank: u8,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering{
        /* Not same rank, go by ranking */
        if self.rank != other.rank {
            return self.rank.cmp(&other.rank);
        }

        for (c, o) in zip(self.cards.chars(), other.cards.chars()) {
            /* Match each card in turn and compare them if not equal */
            if c != o {
                return order(o).cmp(&order(c));
            }
        }
        Ordering::Equal
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Hand {}

impl PartialEq for Hand {
    /* Let's just assume we won't have the same hand twice */
    fn eq(&self, _: &Self) -> bool {
        false
    }
}

fn order(c: char) -> u8 {
    /* Set an ordinal value for each possible card */
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 0, /* Joker is the weakest */
        'T' => 10,
        _ => c.to_string().parse::<u8>().unwrap(),
    }
}

fn find_rank(cards: &String) -> u8 {
    /* Keep track of what sorts of hands we've found */
    let mut five = false;
    let mut four = false;
    let mut three = false;
    let mut pairs = 0;
    let mut jokers = 0;

    let mut sorted: Vec<char> = cards.chars().collect();
    sorted.sort_by(|a, b| b.cmp(a));

    let mut sequential = 1;
    let mut last_char = sorted[0];
    if last_char == 'J' {
        jokers += 1;
    }
    for c in &sorted[1..] {
        if *c == 'J' {
            jokers += 1;
        } else {
            if last_char == *c {
                /* Count up how many cards in a row are the same card */
                sequential += 1;
            } else {
                /* New card, set some info about what we've found */
                match sequential {
                    5 => five = true,
                    4 => four = true,
                    3 => three = true,
                    2 => pairs += 1,
                    _ => ()
                };
                last_char = *c;
                sequential = 1;
            }
        }
    }
    /* Just in case there are sequental cards at the end */
    match sequential {
        5 => five = true,
        4 => four = true,
        3 => three = true,
        2 => pairs += 1,
        _ => ()
    };
    /* Now set a rank based on what sort of hand we have. */
    if five {
       return 1; // Five of a kind
    }

    if four {
        if jokers == 1 {
            return 1; // Five of a kind
        }
        return 2; // Four of a kind
    }
    
    if three {
        return match jokers {
            2 => 1, // Five of a kind
            1 => 2, // Four of a kind
            _ => {
                match pairs {
                    1 => 3, // Full house
                    _ => 4, // Three of a kind
                }
            }
        }
    }

    if pairs > 0 {
        if pairs == 2 {
            if jokers > 0 {
                return 3; // Full house
            }
            return 5; // Two pair
        } else {
            return match jokers {
                3 => 1, // Five of a kind
                2 => 2, // Four of a kind
                1 => 4, // Three of a kind
                _ => 6 // Pair
            }
        }
    }

    match jokers {
        5 => 1, // Five of a kind
        4 => 1, // Five of a kind with non joker
        3 => 2, // Four of a kind with non joker
        2 => 4, // Three of a kind with non joker
        1 => 6, // Pair with non joker
        _ => 7, // Nothing
    }
}

impl Hand {
    pub fn new(cards: String, bid: u32) -> Self {
        /* Compute the rank */
        let rank = find_rank(&cards);
        Self {
            cards,
            bid,
            rank,
        }
    }
}

fn main() {
    let lines = io::stdin().lines();
    let mut hands: Vec<Hand> = Vec::new();
    for maybe_line in lines {
        let line = maybe_line.unwrap();
        /* Parse the hand */
        let mut split = line.split(" ");
        let cards = split.next().unwrap();
        let bid = split.next().unwrap().parse::<u32>().unwrap();
        let hand = Hand::new(cards.to_string(), bid);
        hands.push(hand);
    }

    /* Sort and reverse */
    hands.sort();
    hands.reverse();

    /* Compute how much money each hand is worth */
    let total = hands
        .iter()
        .enumerate()
        .map(|(i, h)| (i + 1) as u32 * h.bid)
        .fold(0, |acc, t| acc + t);

    println!("{}", total);
}

