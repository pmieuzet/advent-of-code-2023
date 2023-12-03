use std::io::Read;

#[derive(Debug)]
struct Gear {
    coordinates: (usize, usize),
    number1: u64,
    number2: u64,
}

fn is_gear(tab: Vec<Vec<char>>, i: usize, j: usize) -> Option<(usize, usize)> {
    let size_tab = tab.len() - 1;
    let size_line = tab[i].len();
    if i != 0 {
        if j != 0 && tab[i - 1][j - 1] == '*' {
            return Some((1 - i, j - 1));
        }
        if tab[i - 1][j] == '*' {
            return Some((i - 1, j));
        }
        if j < size_line - 1 && tab[i - 1][j + 1] == '*' {
            return Some((i - 1, j + 1));
        }
    }

    if i < size_tab - 1 {
        if j != 0 && tab[i + 1][j - 1] == '*' {
            return Some((i + 1, j - 1));
        }
        if tab[i + 1][j] == '*' {
            return Some((i + 1, j));
        }
        if j < size_line - 1 && tab[i + 1][j + 1] == '*' {
            return Some((i + 1, j + 1));
        }
    }

    if j != 0 && tab[i][j - 1] == '*' {
        return Some((i, j - 1));
    }

    if j < size_line - 1 && tab[i][j + 1] == '*' {
        let res = (i, j + 1);
        return Some((i, j + 1));
    }

    None
}

fn is_valid_number(tab: Vec<Vec<char>>, i: usize, size: usize, end_j: usize) -> Option<Gear> {
    let beg_j = end_j - size;
    for j in beg_j..(end_j) {
        if let Some(coordinates) = is_gear(tab.clone(), i, j) {
            let gear = Gear {
                coordinates: coordinates,
                number1: 0,
                number2: 0,
            };
            return Some(gear);
        }
    }

    None
}

fn resolve_engine_schematic(tab: Vec<Vec<char>>) -> u64 {
    let mut is_new_gear = true;
    let mut result_nb = 0;
    let mut gears: Vec<Gear> = Vec::new();
    let mut actual_nb = String::new();

    for i in 0..(tab.len()) {
        for mut j in 0..(tab[i].len()) {
            if tab[i][j].is_digit(10) {
                actual_nb.push(tab[i][j]);
            }
            if (!tab[i][j].is_digit(10) || j == tab[i].len() - 1) && !actual_nb.is_empty() {
                if j == (tab[i].len() - 1) && tab[i][j].is_digit(10) {
                    j += 1;
                }
                if let Some(mut new_gear) = is_valid_number(tab.clone(), i, actual_nb.len(), j) {
                    for gear in &mut gears {
                        if gear.coordinates == new_gear.coordinates {
                            gear.number2 = actual_nb.parse::<u64>().expect("error parse nb");
                            is_new_gear = false;
                            break;
                        }
                    }
                    if is_new_gear {
                        new_gear.number1 = actual_nb.parse::<u64>().expect("error parse nb");
                        gears.push(new_gear);
                    }
                }
                actual_nb.clear();
                is_new_gear = true;
            }
        }
    }
    for gear in gears {
        println!("{gear:?}");
        result_nb += gear.number1 * gear.number2;
    }
    result_nb
}

fn get_vector_split(input: String) -> Vec<Vec<char>> {
    let tab = input.split('\n').collect::<Vec<&str>>();
    let tab_chars: Vec<Vec<char>> = tab
        .iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    tab_chars
}

pub fn part2() {
    let file = std::fs::File::open("input.txt").unwrap();
    let mut reader = std::io::BufReader::new(file);
    let mut input = String::new();

    reader
        .read_to_string(&mut input)
        .expect("Error read input to string");

    let tab = get_vector_split(input);
    let result = resolve_engine_schematic(tab);
    println!("result -> {result}");
}
