use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

fn calculate_scratchcards(scratchcard: String) -> u64 {
    let Some((winners_numbers, my_numbers)) = scratchcard.trim().split_once('|') else {
        println!("error split numbers");
        return 0;
    };
    let winners_numbers = winners_numbers
        .trim()
        .split_ascii_whitespace()
        .map(|number| number.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    let my_numbers = my_numbers
        .trim()
        .split_ascii_whitespace()
        .map(|number| number.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    winners_numbers.iter().fold(0, |acc, win_number| {
        if my_numbers.contains(win_number) {
            if acc == 0 {
                return acc + 1;
            } else {
                return acc * 2;
            }
        }
        acc
    })
}

pub fn part1() {
    let file = std::fs::File::open("input.txt").unwrap();
    let reader = std::io::BufReader::new(file);

    let mut result = 0;

    for line in reader.lines() {
        match line {
            Ok(str) => {
                if let Some(card) = str.split_once(':') {
                    result += calculate_scratchcards(card.1.to_string());
                }
            }
            Err(error) => println!("Error{error}"),
        }
    }
    println!("result = {result}");
}
