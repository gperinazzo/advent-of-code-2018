use std::collections::HashMap;
use std::io::{stdin, BufRead, BufReader};

fn main() {
    let buffer = BufReader::new(stdin());
    let mut two = 0;
    let mut three = 0;
    for line in buffer.lines().filter_map(Result::ok) {
        println!("Processing line {}", &line);
        let mut has_two = false;
        let mut has_three = false;
        let mut letter_map: HashMap<char, u32> = HashMap::new();
        for letter in line.chars() {
            letter_map
                .entry(letter)
                .and_modify(|v| *v += 1)
                .or_insert(1);
        }
        for occurrences in letter_map.values() {
            has_two = has_two || *occurrences == 2;
            has_three = has_three || *occurrences == 3;
        }
        if has_two {
            two += 1;
        }
        if has_three {
            three += 1;
        }
    }
    println!("{}", two * three);
}
