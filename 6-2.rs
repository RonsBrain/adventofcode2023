use std::io;

fn main() {
    let mut buffer = String::new();
    /* Read each of the times */
    let _ = io::stdin().read_line(&mut buffer);
    let time_string: String = buffer[7..]
        .split(" ")
        .filter(|x| !x.trim().is_empty())
        .collect();
    let time: u64 = time_string.trim().parse().unwrap();

    buffer = String::new();
    let _ = io::stdin().read_line(&mut buffer);
    let distance_string: String = buffer[9..]
        .split(" ")
        .filter(|x| !x.trim().is_empty())
        .collect();
    let distance: u64 = distance_string.trim().parse().unwrap();

    let mut ways: u64 = 0;
    for held in 1..time {
        if held * (time - held) > distance {
            ways += 1;
        }
    }

    println!("{}", ways);
}
