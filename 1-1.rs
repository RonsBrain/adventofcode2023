use std::io;

fn main() {
    let lines = io::stdin().lines();
    let mut total = 0;
    for maybe_line in lines {
        let line = maybe_line.unwrap();
        let first = line.chars().find(|&x| x.is_ascii_digit()).unwrap();
        let last = line.chars().rev().find(|&x| x.is_ascii_digit()).unwrap();
        total += first.to_digit(10).unwrap() * 10 + last.to_digit(10).unwrap();
    }
    println!("{}", total);
}
