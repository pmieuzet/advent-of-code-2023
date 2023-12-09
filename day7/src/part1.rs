use std::{collections::HashMap, io::BufRead};

#[derive(Debug, Clone)]
enum Label {
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
    bid: u64,
}

fn get_card_values() -> HashMap<char, u64> {
    HashMap::<char, u64>::from([
        ('A', 0),
        ('K', 1),
        ('Q', 2),
        ('J', 3),
        ('T', 4),
        ('9', 5),
        ('8', 6),
        ('7', 7),
        ('6', 8),
        ('5', 9),
        ('4', 10),
        ('3', 11),
        ('2', 12),
    ])
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
        5 => return Label::HIGHCARD,
        4 => return Label::ONEPAIR,
        3 => return get_size_of_three(winnings),
        2 => return get_size_of_two(winnings),
        1 => return Label::FIVEKING,
        _ => panic!("card cannot have no label"),
    }
}

fn get_index_by_label(cards: String) -> Label {
    let mut winnings = HashMap::<char, i8>::new();
    for mut card in cards.chars() {
        if let Some(actualCard) = winnings.get_mut(&card) {
            *actualCard += 1;
            continue;
        }
        winnings.insert(card, 1);
    }
    return get_label(winnings);
}

fn get_winnings(hands: Vec<Hand>, mut rank: u64) -> u64 {
    let mut winnings = 0;
    for hand in hands {
        winnings += hand.bid * rank;
        rank += 1;
    }
    winnings
}

fn bubble_sort(mut hands: Vec<Hand>) -> Vec<Hand> {
    let card_value = get_card_values();
    let mut i = 0;

    if hands.len() < 2 {
        return hands;
    }

    while i < hands.len() - 1 {
        let mut card1 = hands[i].card.chars();
        let mut card2 = hands[i + 1].card.chars();
        for _ in 0..5 {
            let value1 = card_value.get(&(card1.next().unwrap() as char));
            let value2 = card_value.get(&(card2.next().unwrap() as char));
            if value1 > value2 {
                i += 1;
                break;
            }
            if value1 < value2 {
                hands.swap(i, i + 1);
                i = 0;
                break;
            }
        }
    }
    hands
}

fn calculate_winnings(categories_hands: &Vec<Vec<Hand>>) -> u64 {
    let mut rank: usize = 1;
    let mut winnings = 0;
    for hands in categories_hands {
        let sorted_hands = bubble_sort(hands.clone());
        /*println!(
            "New group: {:?}",
            sorted_hands
                .iter()
                .map(|h| format!("{}", h.card))
                .collect::<Vec<String>>()
        );*/

        for hand in sorted_hands {
            let value = hand.bid as usize * rank;
            winnings += hand.bid as usize * rank;
            println!("{}", hand.card);
            // println!("{} * {} = {} ", hand.bid, rank, winnings);
            rank += 1;
        }
    }
    winnings as u64
}

pub fn part1() {
    let file = std::fs::File::open("input.txt").unwrap();
    let reader = std::io::BufReader::new(file);
    let mut categories_hands = Vec::<Vec<Hand>>::new();

    for _ in 0..(7) {
        categories_hands.push(Vec::<Hand>::new());
    }

    for line in reader.lines() {
        match line {
            Ok(str) => {
                if let Some((card, bid)) = str.split_once(' ') {
                    let index = get_index_by_label(card.to_string());
                    let hand = Hand {
                        card: card.to_string(),
                        bid: bid.parse::<u64>().unwrap(),
                    };
                    //println!("{index:?}");
                    categories_hands[index as usize].push(hand);
                }
            }
            Err(error) => println!("Error{error}"),
        }
    }
    let winnings = calculate_winnings(&categories_hands);
    println!("{winnings}");
}
