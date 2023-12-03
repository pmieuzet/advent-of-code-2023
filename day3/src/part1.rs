use std::io::Read;

fn is_symbol(tab: Vec<Vec<char>>, i: usize, j: usize) -> bool {
    let symbols = "@#$%&*-+=/";

    if let Some(place) = symbols.find(tab[i][j]) {
        return true;
    };
    false
}

fn validity_nb(tab: Vec<Vec<char>>, i: usize, j: usize) -> bool {
    let size_tab = tab.len() - 1;
    let size_line = tab[i].len();
    if i != 0 {
        if j != 0 && is_symbol(tab.clone(), i - 1, j - 1) {
            return true;
        }
        if is_symbol(tab.clone(), i - 1, j) {
            return true;
        }
        if j < size_line - 1 && is_symbol(tab.clone(), i - 1, j + 1) {
            return true;
        }
    }

    if i < size_tab - 1 {
        if j != 0 && is_symbol(tab.clone(), i + 1, j - 1) {
            return true;
        }
        if is_symbol(tab.clone(), i + 1, j) {
            return true;
        }
        if j < size_line - 1 && is_symbol(tab.clone(), i + 1, j + 1) {
            return true;
        }
    }

    if j != 0 && is_symbol(tab.clone(), i, j - 1) {
        return true;
    }

    if j < size_line - 1 && is_symbol(tab.clone(), i, j + 1) {
        return true;
    }

    false
}

fn resolve_engine_schematic(tab: Vec<Vec<char>>) -> u64 {
    let mut is_valid_nb = false;
    let mut result_nb = 0;
    let mut actual_nb = String::new();

    for i in 0..(tab.len()) {
        for mut j in 0..(tab[i].len()) {
            if tab[i][j].is_digit(10) {
                actual_nb.push(tab[i][j]);
                if validity_nb(tab.clone(), i, j) {
                    is_valid_nb = true;
                }
            }
            if (!tab[i][j].is_digit(10) || j == tab[i].len() - 1) && !actual_nb.is_empty() {
                if is_valid_nb {
                    result_nb += actual_nb.parse::<u64>().expect("error parsing number");
                    println!("-> {i}-{j} = {actual_nb}");
                    let len = tab[i].len();
                    is_valid_nb = false;
                }
                actual_nb.clear();
            }
        }
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

pub fn part1() {
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
