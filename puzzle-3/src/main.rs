#[macro_use]
extern crate text_io;

use std::collections::HashMap;
use std::io::{stdin, BufRead, BufReader};

type Position = (u32, u32);
type FabricMap = HashMap<Position, Vec<u32>>;

fn main() {
    let input = BufReader::new(stdin());
    let mut fabric = FabricMap::new();
    for line in input.lines().filter_map(Result::ok) {
        let (id, x, y, width, height): (u32, u32, u32, u32, u32);
        scan!(line.bytes() => "#{} @ {},{}: {}x{}", id, x, y, width, height);
        for i in (x + 1)..=(x + width) {
            for j in (y + 1)..=(y + height) {
                let pos = (i, j);
                fabric
                    .entry(pos)
                    .and_modify(|v| v.push(id))
                    .or_insert(vec![id]);
            }
        }
    }
    let result: usize = fabric.values().map(|v| v.len()).filter(|l| *l > 1).count();
    println!("{}", result);
}
