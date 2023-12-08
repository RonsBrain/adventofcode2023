use std::io;
use std::iter::zip;

fn main() {
    let mut total: u32 = 1;

    let mut buffer = String::new();
    /* Read each of the times */
    let _ = io::stdin().read_line(&mut buffer);
    let time: u32 = buffer[7..]
        .split(" ")
        .filter(|x| !x.trim().is_empty())
        .map(|x| x.trim().parse::<u32>().unwrap())
        .collect();

    /* Read each of the distances */
    buffer = String::new();
    let _ = io::stdin().read_line(&mut buffer);
    let distances: Vec<u32> = buffer[9..]
        .split(" ")
        .filter(|x| !x.trim().is_empty())
        .map(|x| x.trim().parse::<u32>().unwrap())
        .collect();

    /* There's probably some mathy way to figure this out, but let's
     * just brute force it.
     */
    for (time, distance) in zip(times, distances) {
        let mut ways: u32 = 0;
        for held in 1..time {
            if held * (time - held) > distance {
                ways += 1;
            }
        }
        total *= ways;
    }

    println!("{}", total);
}
