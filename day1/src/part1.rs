use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn extract_last(str: String) -> char {
    let mut n: char = '0';
    for i in (0..str.len()).rev() {
        n = str.as_bytes()[i] as char;
        if n.is_digit(10) {
            break;
        }
    }
    n
}

pub fn extract_first(str: String) -> char {
    let mut n: char = '0';
    for i in 0..(str.len()) {
        n = str.as_bytes()[i] as char;
        if n.is_digit(10) {
            return n;
        }
    }
    n
}

pub fn extract_nbr(str: String) -> u64 {
    let mut nbr = extract_first(str.clone()).to_string();
    nbr.push_str(&extract_last(str.clone()).to_string());

    nbr.parse::<u64>().expect(&format!("error parsing nb"))
}

pub fn part1(reader: BufReader<File>) {
    let mut total: u64 = 0;

    for line in reader.lines() {
        match line {
            Ok(str) => total += extract_nbr(str),
            Err(error) => println!("Error{error}"),
        }
    }
    println!("{total}");
}
