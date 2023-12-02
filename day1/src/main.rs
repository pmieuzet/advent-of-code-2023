mod part1;
mod part2;
use std::io::{self};

fn main() {
    let file = std::fs::File::open("input.txt").unwrap();
    let reader = io::BufReader::new(file);
    // part1::part1(reader);
    part2::part2(reader);
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_add() {
        let str = String::from("cnine1");
        assert_eq!(part2::extract_nbr(str), 91);
    }
}
