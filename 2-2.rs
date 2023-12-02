use std::io;
use std::cmp;

fn main() {
    let lines = io::stdin().lines();

    let mut total = 0;

    for maybe_line in lines {
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;

        let line = maybe_line.unwrap();

        /* Get game ID info and pull info */
        let mut broken = line.split(": ");

        /* Skip game ID info */
        broken.next();

        let pulls = broken.next().unwrap();

        /* Split up the pull sets */
        for pull in pulls.split("; ") {
            /* Split up each cube color */
            for cube_pull in pull.split(", ") {
                /* Parse out the color and count */
                let mut outcome = cube_pull.split(" ");
                let count = outcome.next().unwrap().parse::<i32>().unwrap();
                let color = outcome.next().unwrap();
                /* Update the minimum possible cubes for a color by finding
                 * the maximum between the known mimimum and the color count.
                 */
                match color {
                    "red" => max_red = cmp::max(count, max_red),
                    "green" => max_green = cmp::max(count, max_green),
                    "blue" => max_blue = cmp::max(count, max_blue),
                    _ => panic!("Bad color {}", color),
                };
            }
        }
        total += max_red * max_green * max_blue;
    }
    println!("{}", total);
}
