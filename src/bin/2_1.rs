use std::io::prelude::*;

fn main() -> Result<(), failure::Error> {
    let mut line = String::with_capacity(4096);

    std::io::stdin().read_to_string(&mut line)?;

    let input = line.split_whitespace().collect::<Vec<_>>();

    let (mut two, mut three) = (0, 0);

    for line in &input {
        let mut counts = [0; 256];

        for c in line.bytes() {
            counts[c as usize] += 1;
        }

        if counts[('a' as usize)..=('z' as usize)]
            .iter()
            .any(|&c| c == 2)
        {
            two += 1;
        }

        if counts[('a' as usize)..=('z' as usize)]
            .iter()
            .any(|&c| c == 3)
        {
            three += 1;
        }
    }

    println!("{}", two * three);

    Ok(())
}
