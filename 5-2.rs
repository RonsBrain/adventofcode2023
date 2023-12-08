use std::io;
use std::collections::HashSet;
use std::iter::FromIterator;
use std::cmp::{min, max};

/* Yes, there is a std::ops::Range, but it doesn't implement Copy
 * and we want to do fancier things that it can.
 */
#[derive(Eq, PartialEq, Copy, Clone, Hash, Debug)]
struct Range {
    pub start: u64,
    pub end: u64,
}

impl Range {
    pub fn new(start: u64, length: u64) -> Self {
        Self {
            start,
            end: start + length - 1,
        }
    }

    /* Determine the overlapping parts of this range to another,
     * if there is one.
     */
    pub fn overlap(self, other: &Range) -> Option<Self> {
        match (self.start > other.end) || (self.end < other.start) {
            true => None,
            false => Some(Self {
                start: max(self.start, other.start),
                end: min(self.end, other.end),
            })
        }
    }

    /* Given a range, return the part of this range before and after, if
     * there is any remainder
     */
    pub fn remainder(self, other: &Range) -> (Option<Self>, Option<Self>) {
        let before = match other.start > self.start {
            true => Some(Self {
                start: self.start,
                end: other.start - 1,
            }),
            false => None,
        };

        let after = match other.end < self.end {
            true => Some(Self {
                start: other.end + 1,
                end: self.end,
            }),
            false => None,
        };

        (before, after)
    }

    /* How big the range is */
    pub fn len(self) -> u64 {
        self.end - self.start + 1
    }
}

fn main() {
    let mut lines = io::stdin().lines();

    /* Seeds are all on one line. */
    let seed_line = lines.next().unwrap().unwrap();
    /* Skip the next line. */
    lines.next();

    /* Parse out the seed ranges */
    let mut destinations: HashSet<Range> = seed_line[7..]
        .split(" ")
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>()
        .as_slice()
        .chunks(2)
        .map(|x| Range::new(x[0], x[1]))
        .collect();
    let mut sources: HashSet<Range> = HashSet::new();

    loop {
        let skip = lines.next();
        if !skip.is_some() {
            /* EOF */
            break;
        }

        sources = HashSet::from_iter(destinations.clone().into_iter());
        destinations.clear();


        while let Some(maybe_line) = lines.next() {
            let line = maybe_line.unwrap();
            if line.is_empty() {
                /* We're done reading this section. Whatever we didn't map
                 * goes to the destinations as-is.
                 */
                for source in &sources {
                    destinations.insert(*source);
                }
                break;
            }

            /* Parse the numbers */
            let mut numbers = line.split(" ");
            let dest_start = numbers.next().unwrap().parse::<u64>().unwrap();
            let source_start = numbers.next().unwrap().parse::<u64>().unwrap();
            let range = numbers.next().unwrap().parse::<u64>().unwrap();

            /* Make a range for the source so we can do some comparisons */
            let source_range = Range::new(source_start, range);
            let sources_copy: HashSet<Range> = HashSet::from_iter(sources.iter().copied());
            for source in sources_copy.iter() {
                if let Some(overlap) = source_range.overlap(&source) {
                    /* This source overlaps the current range. Remove it from
                     * the set of sources, add the remainder of the source
                     * compared to the current range, then add the mapped
                     * overlap to the destinations.
                     */
                    sources.remove(&source);
                    let (before, after) = source.remainder(&source_range);
                    if before.is_some() {
                        sources.insert(before.unwrap());
                    }
                    if after.is_some() {
                        sources.insert(after.unwrap());
                    }
                    let mapped = Range::new(
                        dest_start + (overlap.start - source_range.start),
                        overlap.len(),
                    );
                    destinations.insert(mapped);
                }
            }
        }
    }
    /* Whatever is left over, goes into destinations as-is */
    for source in &sources {
        destinations.insert(*source);
    }
    println!("{}", destinations.iter().map(|x| x.start).min().unwrap());
}
