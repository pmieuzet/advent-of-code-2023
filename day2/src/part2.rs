use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

struct Party {
    red: u64,
    green: u64,
    blue: u64,
}

fn calculate_party(str: String) -> Party {
    let mut party = Party {
        red: 0,
        green: 0,
        blue: 0,
    };
    let split = str.trim().split_once(' ');
    println!("{split:?}");
    if let Some((number, color)) = split {
        match color {
            "red" => party.red += number.parse::<u64>().unwrap(),
            "blue" => party.blue += number.parse::<u64>().unwrap(),
            "green" => party.green += number.parse::<u64>().unwrap(),
            _ => panic!("error"),
        }
    }
    party
}

fn calculate_game(str: String) -> u64 {
    let mut result = Party {
        red: 0,
        green: 0,
        blue: 0,
    };
    let split = str.split(';');
    for s in split {
        let parties = s.split(',');
        for party in parties {
            let actual_party = calculate_party(party.to_string());
            if actual_party.red > result.red {
                result.red = actual_party.red;
            } else if actual_party.blue > result.blue {
                result.blue = actual_party.blue;
            } else if actual_party.green > result.green {
                result.green = actual_party.green;
            }
        }
    }
    result.red * result.blue * result.green
}

fn get_max_by_game(str: String) -> u64 {
    let split = str.split(':');
    let mut first_time = true;
    let mut result = 0;

    for s in split {
        if first_time {
            first_time = false;
        } else {
            result += calculate_game(s.to_string());
        }
    }
    result
}

pub fn part2() {
    let file = std::fs::File::open("input.txt").unwrap();
    let reader = std::io::BufReader::new(file);

    let mut result = 0;

    for line in reader.lines() {
        match line {
            Ok(str) => result += get_max_by_game(str),
            Err(error) => println!("Error{error}"),
        }
    }
    println!("result = {result}");
}
