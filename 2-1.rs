use std::io;

fn main() {
    let lines = io::stdin().lines();

    let total_red = 12;
    let total_green = 13;
    let total_blue = 14;

    let mut total = 0;

    for maybe_line in lines {
        let line = maybe_line.unwrap();
        /* Get game ID info and pull info */
        let mut broken = line.split(": ");

        /* Parse out game ID */
        let gameset = broken.next().unwrap();
        let mut gamesplit = gameset.split(" ");
        gamesplit.next();
        let game_id = gamesplit.next().unwrap().parse::<i32>().unwrap();

        let pulls = broken.next().unwrap();

        let mut possible = true;
        /* Split up the pull sets */
        for pull in pulls.split("; ") {

            /* Split up each cube color */
            for cube_pull in pull.split(", ") {
                /* Parse out the color and count */
                let mut outcome = cube_pull.split(" ");
                let count = outcome.next().unwrap().parse::<i32>().unwrap();
                let color = outcome.next().unwrap();

                /* We could probably optimize to terminate when we
                 * determine that a game is impossible, but why
                 * bother when the input size is so small?
                 */
                possible = possible && match color {
                    "red" => count <= total_red,
                    "green" => count <= total_green,
                    "blue" => count <= total_blue,
                    _ => panic!("Bad color {}", color),
                };
            }
        }
        if possible {
            total += game_id;
        }
    }
    println!("{}", total);
}
