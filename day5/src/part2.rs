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

fn calculate_result(mut seeds: Vec<(i64, i64)>) -> i64 {
    let mut result = i64::MAX;
    for seed in seeds.into_iter() {
        if seed.0 < result {
            result = seed.0
        }
    }
    result
}

fn get_seeds_positions(seeds: &mut [(i64, i64)], params: Vec<Vec<Params>>) -> Vec<(i64, i64)> {
    for mut seed in seeds.into_iter() {
        let mut min_position = i64::MAX;
        for i in 0..(seed.1) {
            let position = calculate_seed_position(seed.0 + i, &params);
            if position < min_position {
                min_position = position;
            }
        }
        seed.0 = min_position;
        println!("{min_position}");
    }
    seeds.to_vec()
}

fn calculate_seed_position(mut seed: i64, params: &Vec<Vec<Params>>) -> i64 {
    for step in params {
        for param in step {
            if seed >= param.src && seed < (param.src + param.range) {
                let n = param.src + param.range - 1 - seed;
                seed = param.dest + param.range - 1 - n;
                if seed == 0 {
                    println!("{param:?}->{seed}");
                }
                break;
            }
        }
    }
    //println!("seed-> {seed}");
    seed
}

pub fn part2() {
    let file = std::fs::File::open("input.txt").unwrap();
    let reader = std::io::BufReader::new(file);
    let mut next = 0;
    let mut seeds = Vec::<(i64, i64)>::new();
    let mut step = Vec::<Params>::new();
    let mut params = Vec::<Vec<Params>>::new();

    for line in reader.lines() {
        match line {
            Ok(str) => {
                if next == 0 {
                    if let Some((name, values)) = str.split_once(':') {
                        let tmp = values
                            .trim()
                            .split_ascii_whitespace()
                            .map(|elem| elem.parse::<i64>().unwrap())
                            .collect::<Vec<i64>>();
                        let mut i = 0;
                        while (i < tmp.len()) {
                            seeds.push((tmp[i], tmp[i + 1]));
                            i += 2;
                        }
                        println!("first:{seeds:?}")
                    }
                    next += 1;
                } else if str.find(":").is_some() {
                    continue;
                } else if str.len() < 2 {
                    params.push(step.clone());
                    step.clear();
                    continue;
                } else {
                    let element = str
                        .trim()
                        .split_ascii_whitespace()
                        .map(|elem| elem.parse::<i64>().unwrap())
                        .collect::<Vec<i64>>();
                    step.push(Params {
                        dest: element[0],
                        src: element[1],
                        range: element[2],
                    });
                }
            }
            Err(error) => println!("Error{error}"),
        }
    }
    params.push(step.clone());
    seeds = get_seeds_positions(&mut seeds, params);
    let result = calculate_result(seeds);
    println!("{result}");
}
