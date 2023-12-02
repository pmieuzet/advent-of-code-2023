use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead, BufReader},
};

fn is_possible_game(str: String) -> bool {
    let split = str.split(';');
    for s in split {
        if !is_possible_party(s.to_string()) {
            return false;
        }
    }
    true
}

fn is_possible_party(str: String) -> bool {
    let split = str.split(',');
    for s in split {
        if !is_valide_party(s.to_string()) {
            return false;
        }
    }
    true
}

fn check_verify(red: u32, green: u32, blue: u32) -> bool {
    if red > 12 || green > 13 || blue > 14 {
        return false;
    }
    true
}

struct Party {
    red: u32,
    green: u32,
    blue: u32,
}

fn is_valide_party(str: String) -> bool {
    let mut party = Party {
        red: 0,
        green: 0,
        blue: 0,
    };
    let split = str.trim().split_once(' ');
    println!("{split:?}");
    if let Some((number, color)) = split {
        match color {
            "red" => party.red += number.parse::<u32>().unwrap(),
            "blue" => party.blue += number.parse::<u32>().unwrap(),
            "green" => party.green += number.parse::<u32>().unwrap(),
            _ => panic!("error"),
        }
    }
    check_verify(party.red, party.green, party.blue)
}

fn get_index(str: String) -> u64 {
    let mut first_time = true;
    let split = str.split_once(' ');
    if let Some((left, right)) = split {
        return right.parse::<u64>().expect("error parse");
    }
    0
}

fn calculate_game(str: String) -> u64 {
    let split = str.split(':');
    let mut first_time = true;
    let mut index = 0;

    for s in split {
        if first_time {
            index = get_index(s.to_string());
            first_time = false;
        } else if is_possible_game(s.to_string()) {
            return index;
        } else {
            return 0;
        }
    }
    0
}

pub fn part1() {
    let file = std::fs::File::open("input.txt").unwrap();
    let reader = std::io::BufReader::new(file);

    let mut result = 0;

    for line in reader.lines() {
        match line {
            Ok(str) => result += calculate_game(str),
            Err(error) => println!("Error{error}"),
        }
    }
    println!("result = {result}");
}
