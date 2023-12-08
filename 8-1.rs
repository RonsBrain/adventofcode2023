use std::io;
use std::collections::HashMap;

fn main () {
    let mut lines = io::stdin().lines();

    /* Get directions, skip next line */
    let direction_line = lines.next().unwrap().unwrap();
    lines.next();
    let mut network: HashMap<String, (String, String)> = HashMap::new();

    /* Parse the network. Don't bother with fancy splits; input line is
     * always the same size.
     */
    for maybe_line in lines {
        let line = maybe_line.unwrap();
        let key = line[0..3].to_string();
        let value = (line[7..10].to_string(), line[12..15].to_string());
        network.insert(key, value);
    }

    /* Start at AAA */
    let mut current = String::from("AAA");
    let mut steps = 0;
    for (i, direction) in direction_line.chars().cycle().enumerate() {
        /* Find each direction and choose the next location */
        let fork = network.get(&current).unwrap();
        let next = match direction {
            'L' => &fork.0,
            'R' => &fork.1,
            _ => panic!("??? {}", direction),
        };
        if next == "ZZZ" {
            /* We found it! */
            steps = i;
            break;
        }
        current = String::from(next);
    };
    println!("{:?}", steps + 1);
}
