use std::{
    fs::{self, File},
    io::{self, BufRead, BufReader},
};

#[derive(Debug, Clone)]
struct Race {
    time: u32,
    distance: u32,
}

fn get_result(records: Vec<u32>) -> u32 {
    let mut result = 1;
    for record in records.into_iter() {
        result *= record;
    }
    result
}

fn calculate_nb_of_records(races: Vec<Race>) -> u32 {
    let mut result = Vec::<u32>::new();
    for race in races {
        let mut records: u32 = 0;
        for i in 1..=(race.time) {
            if (race.time - i) * (i) > race.distance {
                records += 1;
            }
        }
        result.push(records);
    }
    get_result(result)
}

fn parse_args(str: String) -> Vec<Race> {
    let mut args = Vec::<Race>::new();
    let mut times = Vec::<u32>::new();
    let mut distances = Vec::<u32>::new();
    if let Some((times_str, distances_str)) = str.split_once('\n') {
        if let Some(time_split) = times_str.split_once(':') {
            times = time_split
                .1
                .trim()
                .split_whitespace()
                .map(|elem| elem.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
        }
        if let Some(distances_split) = distances_str.split_once(':') {
            distances = distances_split
                .1
                .trim()
                .split_whitespace()
                .map(|elem| elem.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
        }
    }
    for i in 0..(times.len()) {
        args.push(Race {
            time: times[i],
            distance: distances[i],
        });
    }
    args
}

pub fn part1() {
    let str = fs::read_to_string("input.txt").unwrap();
    let races = parse_args(str);
    let result = calculate_nb_of_records(races);
    println!("{result}");
}
