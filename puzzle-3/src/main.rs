#[macro_use]
extern crate text_io;

use std::collections::{HashMap, HashSet};
use std::io::{stdin, BufRead, BufReader};

type Position = (u32, u32);
type FabricMap = HashMap<Position, Vec<u32>>;

fn main() {
    let input = BufReader::new(stdin());
    let mut fabric = FabricMap::new();
    let mut overlapped_claims: HashMap<u32, bool> = HashMap::new();
    for line in input.lines().filter_map(Result::ok) {
        let mut colliding_ids: HashSet<u32> = HashSet::new();
        let (id, x, y, width, height): (u32, u32, u32, u32, u32);
        scan!(line.bytes() => "#{} @ {},{}: {}x{}", id, x, y, width, height);
        for i in (x + 1)..=(x + width) {
            for j in (y + 1)..=(y + height) {
                let pos = (i, j);
                fabric
                    .entry(pos)
                    .and_modify(|v| {
                        colliding_ids.extend(v.iter());
                        v.push(id)
                    })
                    .or_insert(vec![id]);
            }
        }
        overlapped_claims.insert(id, !colliding_ids.is_empty());
        for id in colliding_ids.drain() {
            let entry = overlapped_claims.entry(id).or_default();
            *entry = true;
        }
    }
    let result: usize = fabric.values().map(|v| v.len()).filter(|l| *l > 1).count();
    println!("{}", result);
    for (id, overlapped) in overlapped_claims.iter() {
        if !overlapped {
            println!("{}", id);
            break;
        }
    }
}
