use std::io;
use std::collections::HashSet;
use std::iter::FromIterator;

fn main() {
    let mut lines = io::stdin().lines();

    /* Seeds are all on one line. */
    let seed_line = lines.next().unwrap().unwrap();
    /* Skip the next line. */
    lines.next();

    /* Parse out the seeds numbers */
    let mut destinations: Vec<u64> = seed_line[7..]
        .split(" ")
        .map(|x| x.parse::<u64>().unwrap())
        .collect();
    let mut sources: HashSet<u64> = HashSet::new();

    /* Each mapping section come sequentially, so we can ignore
     * the header for each section and just know that whatever we
     * need to go through each range and see if the previous
     * mapped values are within those ranges. If so, keep track
     * of the mapped value for next iteration. When all done,
     * whatever wasn't mapped needs to be added as-is to the
     * next set of mappings.
     */
    loop {
        let skip = lines.next();
        if !skip.is_some() {
            /* EOF */
            break;
        }
        /* Whatever we mapped from last iteration is now what we need to
         * look up for this iteration.
         */
        sources = HashSet::from_iter(destinations.iter().copied());
        destinations.clear();

        while let Some(maybe_line) = lines.next() {
            let line = maybe_line.unwrap();
            if line.is_empty() {
                /* We're done reading this section. Whatever we didn't map
                 * goes to the destinations as-is.
                 */
                for source in &sources {
                    destinations.push(*source);
                }
                break;
            }

            /* Parse the numbers */
            let mut numbers = line.split(" ");
            let dest_start = numbers.next().unwrap().parse::<u64>().unwrap();
            let source_start = numbers.next().unwrap().parse::<u64>().unwrap();
            let range = numbers.next().unwrap().parse::<u64>().unwrap();

            /* Work on a copy of the things we need to map*/
            let sources_copy: HashSet::<u64> = HashSet::from_iter(sources.iter().copied());

            for source in &sources_copy {
                /* If the source is in range, push it to destinations */
                if (source_start..(source_start + range)).contains(&source) {
                    let dest = dest_start + (source - source_start);
                    sources.remove(&source);
                    destinations.push(dest);
                }
            }
        }
    }

    /* Whatever is left over goes into destinations. */
    for source in &sources {
        destinations.push(*source);
    }

    /* Destinations now contains the final mapped values.
     * Find the smallest.
     */
    println!("{}", destinations.iter().min().unwrap());
}
