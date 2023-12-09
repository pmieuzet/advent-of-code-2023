use std::{
    collections::HashMap,
    fs::{self, File},
    io::{self, BufRead, BufReader},
};

#[derive(Debug, Clone)]
enum Label {
    NONE,
    HIGHCARD,
    ONEPAIR,
    TWOPAIR,
    THREEKING,
    FULLHOUSE,
    FOURKING,
    FIVEKING,
}

#[derive(Debug, Clone)]
struct Hand {
    card: String,
    bid: u32,
    label: Label,
}

fn get_total_winnings(hands: Vec<Hand>) -> u32 {
    for i in 0..=(7) {}
    0
}

fn get_size_of_two(winnings: HashMap<char, i8>) -> Label {
    for win in winnings.values() {
        if *win == 4 {
            return Label::FOURKING;
        }
    }
    Label::FULLHOUSE
}

fn get_size_of_three(winnings: HashMap<char, i8>) -> Label {
    for win in winnings.values() {
        if *win == 3 {
            return Label::THREEKING;
        }
    }
    Label::TWOPAIR
}

fn get_label(winnings: HashMap<char, i8>) -> Label {
    match winnings.len() {
        1 => return Label::FIVEKING,
        5 => return Label::HIGHCARD,
        4 => return Label::ONEPAIR,
        3 => return get_size_of_three(winnings),
        2 => return get_size_of_two(winnings),
        _ => return Label::NONE,
    }
}

fn calculate_winnings(hands: Vec<Hand>) -> u32 {
    for mut hand in hands {
        let mut winnings = HashMap::<char, i8>::new();
        for card in hand.card.chars() {
            if let Some(actualCard) = winnings.get_mut(&card) {
                *actualCard += 1;
            }
            winnings.insert(card, 1);
        }
        hand.label = get_label(winnings);
    }
    0
    // get_total_winnings()
}

pub fn part1() {
    let file = std::fs::File::open("input.txt").unwrap();
    let reader = std::io::BufReader::new(file);
    let mut hands = Vec::<Hand>::new();

    for line in reader.lines() {
        match line {
            Ok(str) => {
                if let Some((card, bid)) = str.split_once(' ') {
                    hands.push(Hand {
                        card: card.to_string(),
                        bid: bid.parse::<u32>().unwrap(),
                        label: Label::NONE,
                    });
                }
            }
            Err(error) => println!("Error{error}"),
        }
    }
    let winnings = calculate_winnings(hands);
    // println!("{winnings}");
}
