use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    result,
};

#[derive(Debug, Clone)]
struct Scratchcard {
    card: usize,
    win_numbers: Vec<u64>,
    my_numbers: Vec<u64>,
}

#[derive(Debug, Clone)]
struct Card {
    index: usize,
    occ: u64,
}

fn get_card_by_index(scratchcards: Vec<Scratchcard>, index: usize) -> Option<Scratchcard> {
    if let Some(card) = scratchcards.get(index) {
        return Some(Scratchcard {
            card: (card.card),
            win_numbers: (card.win_numbers.clone()),
            my_numbers: (card.my_numbers.clone()),
        });
    }
    None
}

fn generate_cards(scractchcards: Vec<Scratchcard>) -> Vec<Card> {
    let mut cards = Vec::<Card>::new();
    for scratchcard in scractchcards {
        let new_card = Card {
            index: scratchcard.card.clone(),
            occ: 1,
        };
        cards.push(new_card);
    }
    cards
}

fn calculate_result(cards: Vec<Card>) -> u64 {
    let mut result = 0;
    for card in cards {
        result += card.occ;
    }
    result
}

fn get_numbers_scratchcards(scractchcards: Vec<Scratchcard>) -> u64 {
    let mut result = 0;
    let mut cards = generate_cards(scractchcards.clone());
    for scratchcard in scractchcards {
        let mut rep = 0;
        for win_number in scratchcard.win_numbers.clone() {
            if scratchcard.my_numbers.contains(&win_number) {
                rep += 1;
            }
        }
        if rep == 0 {
            continue;
        }
        if let Some(card) = cards.clone().get(scratchcard.clone().card) {
            for i in 1..=(rep) {
                if let Some(actual_card) = cards.get_mut(scratchcard.clone().card + i) {
                    actual_card.occ += card.occ.clone();
                    continue;
                }
                result += card.occ.clone();
            }
        }
    }
    result += calculate_result(cards.clone());
    result
}

fn get_new_scratchcards(card: &str, list_of_numbers: &str) -> Option<Scratchcard> {
    let index = card
        .split_once(' ')
        .unwrap()
        .1
        .trim()
        .parse::<usize>()
        .unwrap();
    let Some((winners_numbers, my_numbers)) = list_of_numbers.trim().split_once('|') else {
        println!("error split numbers");
        return None;
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

    Some(Scratchcard {
        card: (index),
        win_numbers: (winners_numbers),
        my_numbers: (my_numbers),
    })
}

pub fn part2() {
    let file = std::fs::File::open("input.txt").unwrap();
    let reader = std::io::BufReader::new(file);
    let mut scractchcards = Vec::<Scratchcard>::new();

    for line in reader.lines() {
        match line {
            Ok(str) => {
                if let Some((card, list_of_numbers)) = str.split_once(':') {
                    if let Some(new_scratchcard) = get_new_scratchcards(card, list_of_numbers) {
                        scractchcards.push(new_scratchcard);
                    }
                }
            }
            Err(error) => println!("Error{error}"),
        }
    }

    let result = get_numbers_scratchcards(scractchcards);
    println!("result = {result}");
}
