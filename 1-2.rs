use std::io;

fn main() {
    let lines = io::stdin().lines();
    let mut total = 0;
    for maybe_line in lines {
        let line = maybe_line.unwrap();
        /* Replace each spelled out digit with the real digit.
         * Note: Spelled out digits can overlap, so keep the
         * first and last letters of the digit if it makes sense.
         */
        let replaced = line.replace("one", "o1e")
            .replace("two", "t2o")
            .replace("three", "t3e")
            .replace("four", "4")
            .replace("five", "5e")
            .replace("six", "6")
            .replace("seven", "7n")
            .replace("eight", "e8t")
            .replace("nine", "n9e");
        let first = replaced.chars().find(|&x| x.is_ascii_digit()).unwrap();
        let last = replaced.chars().rev().find(|&x| x.is_ascii_digit()).unwrap();
        total += first.to_digit(10).unwrap() * 10 + last.to_digit(10).unwrap();
    }
    println!("{}", total);
}
