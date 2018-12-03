use std::collections::HashMap;
use std::io::{stdin, BufRead, BufReader, Read};

fn part_1() {
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

type Key<'a> = (&'a str, &'a str);

fn main() {
    let mut buffer = String::new();
    stdin().read_to_string(&mut buffer).unwrap();
    let mut id_map: HashMap<Key, u32> = HashMap::new();
    for line in buffer.lines() {
        for i in 0..line.len() {
            let key = (&line[..i], &line[(i + 1)..]);
            id_map.entry(key).and_modify(|v| *v += 1).or_insert(1);
        }
    }
    for (key, occurrences) in id_map.iter() {
        if *occurrences > 1 {
            println!("{}: {}{}", occurrences, key.0, key.1);
        }
    }
}
