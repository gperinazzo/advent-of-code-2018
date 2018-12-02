use std::collections::HashSet;
use std::env::args;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::FromIterator;

fn main() {
    let args: Vec<String> = args().collect();
    let mut seen: HashSet<i64> = HashSet::from_iter(vec![0]);
    if args.len() != 2 {
        eprintln!("Usage: advent-of-code-1 <input file>");
        return;
    }
    let f = File::open(&args[1]).unwrap();
    let values: Vec<i64> = BufReader::new(f)
        .lines()
        .filter_map(Result::ok)
        .filter_map(|v| Result::ok(v.parse::<i64>()))
        .collect();
    let mut cur = 0;
    loop {
        for value in values.iter() {
            cur += value;
            if seen.contains(&cur) {
                println!("Seen for the second time: {}", cur);
                return;
            }
            seen.insert(cur);
        }
    }
}
