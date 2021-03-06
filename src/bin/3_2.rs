use std::collections::HashSet;
use std::io::prelude::*;

#[derive(Debug)]
struct Factory {
    id: i32,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
}

fn main() -> Result<(), failure::Error> {
    let mut factories = vec![];

    let mut map = vec![vec![]; 1000 * 1000];

    for line in std::io::stdin().lock().lines() {
        let line = line?;

        // #___ @ ___,___: ___x___
        //   ID     X Y      W H

        let mut parts = line.split(' ');

        // #ID
        let id = parts.next().unwrap();
        let id = id[1..].parse::<i32>()?;

        // @
        parts.next();

        // X,Y:
        let mut x_y = parts.next().unwrap().split(',');
        let x = x_y.next().unwrap();
        let x = x.parse::<i32>()?;
        let y = x_y.next().unwrap();
        let y = y[..y.len() - 1].parse::<i32>()?;

        // WxH
        let mut w_h = parts.next().unwrap().split('x');
        let w = w_h.next().unwrap();
        let w = w.parse::<i32>()?;
        let h = w_h.next().unwrap();
        let h = h.parse::<i32>()?;

        let factory = Factory { id, x, y, w, h };
        factories.push(factory);

        for i in 0..h {
            let start = x + (y + i) * 1000;
            let end = start + w;

            for area in map[start as usize..end as usize].iter_mut() {
                area.push(id);
            }
        }
    }

    let mut overlapping = vec![0; factories.len() + 1];

    for area in &map {
        if area.len() > 1 {
            for id in area {
                overlapping[*id as usize] = 1;
            }
        }
    }

    unimplemented!();

    for (id, val) in overlapping[1..].iter().enumerate() {
        if *val == 0 {
            println!("{}", id);
            break;
        }
    }

    Ok(())
}
