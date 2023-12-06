use core::time;
use std::{
    fs::{self, File},
    io::{self, BufRead, BufReader},
};

#[derive(Debug, Clone)]
struct Race {
    time: u64,
    distance: u64,
}

fn calculate_nb_of_records(race: Race) -> u64 {
    let mut records: u64 = 0;
    for i in 1..=(race.time) {
        if (race.time - i) * (i) > race.distance {
            records += 1;
        }
    }
    records
}

fn parse_args(str: String) -> Race {
    let mut race = Race {
        time: 0,
        distance: 0,
    };
    if let Some((times_str, distances_str)) = str.split_once('\n') {
        if let Some(time_split) = times_str.split_once(':') {
            let times = time_split
                .1
                .trim()
                .split_whitespace()
                .into_iter()
                .fold(String::new(), |acc, str| acc + str);
            race.time = times.parse::<u64>().unwrap();
        }
        if let Some(distances_split) = distances_str.split_once(':') {
            let distances = distances_split
                .1
                .trim()
                .split_whitespace()
                .into_iter()
                .fold(String::new(), |acc, str| acc + str);
            race.distance = distances.parse::<u64>().unwrap();
        }
    }
    race
}

pub fn part2() {
    let str = fs::read_to_string("input.txt").unwrap();
    let races = parse_args(str);
    let result = calculate_nb_of_records(races);
    println!("{result}");
}
