extern crate regex;
#[macro_use]
extern crate lazy_static;
use std::cmp::Ordering;
use std::error::Error;
use std::str::FromStr;
use regex::Regex;
use std::io::{stdin, BufReader, BufRead};
use std::collections::{HashMap};

type SleepMap = HashMap<u32, [u32;60]>;

fn main() {
    let reader = BufReader::new(stdin());
    let mut records: Vec<Record> = reader.lines()
        .filter_map(Result::ok)
        .filter_map(|s| Record::from_str(&s).ok())
        .collect();
    if records.len() == 0 {
       println!("input is empty");
       return;
    }
    records.sort_unstable();        

    let mut map = SleepMap::new(); 
    
    let mut current_guard: u32;
    let mut sleep_start: Option<u8> = None;
    if let Action::StartShift(g) = records[0].action {
        current_guard = g;
    }
    else {
        println!("first record is not a shift");
        return;
    }

    for record in records.iter().skip(1) {
        match record.action {
            Action::StartShift(c) => {
                if let Some(start) = sleep_start {
                    increment_sleep(&mut map, current_guard, start, 60); 
                }
                current_guard = c;
            },
            Action::Sleep => {
                sleep_start = Some(record.minute); 
            },
            Action::Wake => {
                if let Some(start) = sleep_start {
                    increment_sleep(&mut map, current_guard, start, record.minute); 
                }
            }
        }
    }
    let (id, minute) = find_best_guard(&map);        
    println!("{} {} {}", id, minute, id * minute);
}

fn increment_sleep(m: &mut SleepMap, id: u32, start: u8, end: u8) {
    let v = m.entry(id).or_insert([0; 60]);
    for i in start..end {
        v[i as usize] += 1;
    }
}

fn find_best_guard(m: &SleepMap) -> (u32, u32) {
    let (id, _, minute) = m.iter()
        .map(|(id, v)| (
                id,
                v.iter().sum::<u32>(),
                v.iter().enumerate()
                    .max_by_key(|&(_, item)| item).unwrap().0
            )
        ).max_by_key(|&(_, sum, _)| sum).unwrap();
    (*id, minute as u32)
}

type BoxResult<T> = Result<T, Box<Error>>;

#[derive(Debug)]
enum Action {
    StartShift(u32),
    Sleep,
    Wake,
}

#[derive(Debug)]
struct Record {
   timestamp: String,
   minute: u8,
   action: Action,
}

impl Ord for Record {
    fn cmp(&self, other: &Record) -> Ordering {
        match self.timestamp.cmp(&other.timestamp) {
            Ordering::Equal => self.minute.cmp(&other.minute),
            x => x,
        }
    }
}

impl PartialOrd for Record {
    fn partial_cmp(&self, other: &Record) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Record {
}

impl PartialEq for Record {
    fn eq(&self, other: &Record) -> bool {
        self.timestamp == other.timestamp
    }
}

impl FromStr for Record {
    type Err = Box<Error>;

    fn from_str(s: &str) -> BoxResult<Record> {
        lazy_static! {
            static ref TIME_RE: Regex = Regex::new(r"\[([0-9]{4}-[0-9]{2}-[0-9]{2}) [0-9]{2}:([0-9]{2})\]").unwrap();
            static ref SLEEP_RE: Regex = Regex::new(r"falls asleep").unwrap(); 
            static ref WAKE_RE: Regex = Regex::new(r"wakes up").unwrap();
            static ref SHIFT_RE: Regex = Regex::new(r"Guard #([0-9]+) begins shift").unwrap();
        }
        let timestamp;
        let minute;
        if let Some(c) = TIME_RE.captures(s) {
            timestamp = c.get(1).unwrap().as_str().to_string();
            minute = c.get(2).unwrap().as_str().parse::<u8>()?;
        } else {
            return Result::Err("Invalid timestamp".into());
        }

        if WAKE_RE.is_match(s) {
            return Ok(Record{timestamp, minute, action: Action::Wake});
        }

        if SLEEP_RE.is_match(s) {
            return Ok(Record{timestamp, minute, action: Action::Sleep});
        }

        if let Some(c) = SHIFT_RE.captures(s) {
            let id = c.get(1).unwrap().as_str().parse::<u32>()?;
            return Ok(Record{timestamp, minute, action: Action::StartShift(id)});
        }

        Result::Err("Unable to parse action".into())
    }
}

