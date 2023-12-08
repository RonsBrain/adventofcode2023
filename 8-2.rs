use std::io;
use std::collections::{ HashMap, HashSet };

fn lcm(this: u64, that: u64) -> u64 {
    let mut a = this;
    let mut b = that;
    let mut temp: u64;
    /* Compute GCD */
    while b != 0{
        temp = b;
        b = a % b;
        a = temp;
    }
    /* LCM(a, b) = ab / GCD(a, b) */
    (this * that) / a
}

fn main () {
    let mut lines = io::stdin().lines();

    /* Get directions, skip next line */
    let direction_line = lines.next().unwrap().unwrap();
    lines.next();
    let mut network: HashMap<String, (String, String)> = HashMap::new();
    let mut ghosts: HashSet<String> = HashSet::new();

    /* Parse the network. Don't bother with fancy splits; input line is
     * always the same size.
     */
    for maybe_line in lines {
        let line = maybe_line.unwrap();
        let key = line[0..3].to_string();
        let value = (line[7..10].to_string(), line[12..15].to_string());
        if key.ends_with("A") {
            ghosts.insert(key.clone());
        }
        network.insert(key, value);
    }

    let mut ghost_steps: Vec<u64> = Vec::new();
    /* Find how many steps each ghost takes to get to the end */
    for ghost in ghosts {
        let mut current = ghost;
        for (i, direction) in direction_line.chars().cycle().enumerate() {
            /* Find each direction and choose the next location */
            let fork = network.get(&current).unwrap();
            let next = match direction {
                'L' => &fork.0,
                'R' => &fork.1,
                _ => panic!("??? {}", direction),
            };

            if next.ends_with("Z") {
                /* We found it! */
                ghost_steps.push(i as u64 + 1);
                break;
            }
            current = next.to_string();
        }
    };
    /* The total number of steps is the LCM of all the steps for the ghosts */
    let steps = ghost_steps.iter().fold(1, |i,x | lcm(i, *x));
    println!("{:?}", steps);
}
