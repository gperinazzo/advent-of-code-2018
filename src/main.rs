use std::env::args;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() != 2 {
        eprintln!("Usage: advent-of-code-1 <input file>");
        return;
    }
    let f = File::open(&args[1]).unwrap();
    let value: i64 = BufReader::new(f)
        .lines()
        .filter_map(Result::ok)
        .filter_map(|v| Result::ok(v.parse::<i64>()))
        .sum();
    println!("{}", value);
}
