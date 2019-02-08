use std::collections::HashSet;
use std::io::prelude::*;

fn main() -> Result<(), failure::Error> {
    let mut line = String::with_capacity(4096);

    std::io::stdin().read_to_string(&mut line)?;

    let input = line
        .split_whitespace()
        .map(|n| n.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let mut sum = 0;
    let mut seen = HashSet::new();
    seen.insert(0);

    'outer: loop {
        for x in &input {
            sum += x;

            // insert() returns false if `seen` already contains `sum`
            if !seen.insert(sum) {
                println!("{}", sum);

                break 'outer;
            }
        }
    }

    Ok(())
}
