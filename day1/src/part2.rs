use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use super::part1::extract_first;
use super::part1::extract_last;

pub fn extract_index_last(str: String) -> i128 {
    let mut n: char = '0';
    for i in (0..str.len()).rev() {
        n = str.as_bytes()[i] as char;
        if n.is_digit(10) {
            return i as i128;
        }
    }
    -1
}

pub fn extract_index_first(str: String) -> i128 {
    let mut n: char = '0';
    for i in 0..(str.len()) {
        n = str.as_bytes()[i] as char;
        if n.is_digit(10) {
            return i as i128;
        }
    }
    -1
}

fn get_map_digits_string() -> HashMap<String, char> {
    let digits_string = HashMap::from([
        (String::from("zero"), '0'),
        (String::from("one"), '1'),
        (String::from("two"), '2'),
        (String::from("three"), '3'),
        (String::from("four"), '4'),
        (String::from("five"), '5'),
        (String::from("six"), '6'),
        (String::from("seven"), '7'),
        (String::from("eight"), '8'),
        (String::from("nine"), '9'),
    ]);
    digits_string
}

pub fn extract_nbr(str: String) -> u64 {
    let digits_string = get_map_digits_string();
    let mut first = ('a', str.len());
    let mut last = ('a', 0);
    let mut nbr = String::new();
    let mut var = 0;
    let first_index = extract_index_first(str.clone());
    let last_index = extract_index_last(str.clone());

    for key in digits_string.keys() {
        let Some(found) = str.find(key) else {
            continue;
        };
        var = found;

        if first_index < var as i128 && (first_index as usize) <= first.1 {
            first.0 = extract_first(str.clone());
            first.1 = first_index as usize;
        } else if first_index > var as i128 && var <= first.1 {
            first.0 = *digits_string.get(key).expect("error search value");
            first.1 = var;
        }

        if last_index > var as i128 && (last_index as usize) >= last.1 {
            last.0 = extract_last(str.clone());
            last.1 = last_index as usize;
        } else if last_index < var as i128 && var >= last.1 {
            last.0 = *digits_string.get(key).expect("error search value");
            last.1 = var;
        }
        let Some(found) = str.rfind(key) else {
            continue;
        };

        var = found;
        if first_index < var as i128 && (first_index as usize) <= first.1 {
            first.0 = extract_first(str.clone());
            first.1 = first_index as usize;
        } else if first_index > var as i128 && var <= first.1 {
            first.0 = *digits_string.get(key).expect("error search value");
            first.1 = var;
        }

        if last_index > var as i128 && (last_index as usize) >= last.1 {
            last.0 = extract_last(str.clone());
            last.1 = last_index as usize;
        } else if last_index < var as i128 && var >= last.1 {
            last.0 = *digits_string.get(key).expect("error search value");
            last.1 = var;
        }
    }

    if first.0 == 'a' {
        first.0 = extract_first(str.clone());
    }
    if last.0 == 'a' {
        last.0 = extract_last(str.clone());
    }

    nbr.push_str(&first.0.to_string());
    nbr.push_str(&last.0.to_string());
    nbr.parse::<u64>().expect(&format!("error parsing nb"))
}

pub fn part2(reader: BufReader<File>) {
    let mut total: u64 = 0;

    for line in reader.lines() {
        match line {
            Ok(str) => total += extract_nbr(str.clone()),
            Err(error) => println!("Error{error}"),
        }
    }
    println!("{total}");
}
