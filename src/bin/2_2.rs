use std::cmp::Ordering;
use std::io::prelude::*;

fn main() -> Result<(), failure::Error> {
    let mut line = String::with_capacity(4096);

    std::io::stdin().read_to_string(&mut line)?;

    let mut input = line.split_whitespace().collect::<Vec<_>>();

    let mut answer = String::with_capacity(32);

    input.sort();

    input.sort_by(|a, b| {
        if a.bytes().zip(b.bytes()).filter(|(x, y)| x != y).count() == 1 {
            Ordering::Equal
        } else {
            Ordering::Greater
        }
    });

    for line in input.windows(2) {
        let a = line[0];
        let b = line[1];

        if a.bytes().zip(b.bytes()).filter(|(x, y)| x != y).count() == 1 {
            for (x, _) in a.bytes().zip(b.bytes()).filter(|(x, y)| x == y) {
                answer.push(x as char);
            }

            println!("{}", answer);
            break;
        }
    }

    Ok(())
}
