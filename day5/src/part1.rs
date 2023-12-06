use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

#[derive(Debug, Clone)]
struct Params {
    dest: i64,
    src: i64,
    range: i64,
}

fn calculate_result(mut seeds: Vec<i64>) -> i64 {
    println!("{seeds:?}");
    seeds.sort();
    *seeds.first().unwrap()
}

fn calculate_next_step(seeds: &mut [i64], params: &[Params]) -> Vec<i64> {
    for mut seed in seeds.into_iter() {
        for param in params {
            if *seed >= param.src && *seed < (param.src + param.range) {
                let n = param.src + param.range - 1 - *seed;
                *seed = (param.dest + param.range - 1) - n;
                break;
            }
        }
    }
    seeds.to_vec()
}

pub fn part1() {
    let file = std::fs::File::open("input.txt").unwrap();
    let reader = std::io::BufReader::new(file);
    let mut next = 0;
    let mut seeds = Vec::<i64>::new();
    let mut params = Vec::<Params>::new();

    for line in reader.lines() {
        match line {
            Ok(str) => {
                if next == 0 {
                    if let Some((name, values)) = str.split_once(':') {
                        seeds = values
                            .trim()
                            .split_ascii_whitespace()
                            .map(|elem| elem.parse::<i64>().unwrap())
                            .collect::<Vec<i64>>();
                    }
                    //println!("{seeds:?}");
                    next += 1;
                } else if str.find(":").is_some() {
                    continue;
                } else if str.len() < 2 {
                    seeds = calculate_next_step(&mut seeds, &params);
                    println!("{seeds:?}");
                    params.clear();
                    continue;
                } else {
                    let param = str
                        .trim()
                        .split_ascii_whitespace()
                        .map(|elem| elem.parse::<i64>().unwrap())
                        .collect::<Vec<i64>>();
                    params.push(Params {
                        dest: param[0],
                        src: param[1],
                        range: param[2],
                    });
                }
            }
            Err(error) => println!("Error{error}"),
        }
    }
    seeds = calculate_next_step(&mut seeds, &params);
    let result = calculate_result(seeds);
    println!("{result}");
}
