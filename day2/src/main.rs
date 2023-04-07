use std::fs::{self, File};
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;


fn main() {
    let start = Instant::now();
    let part_one_result = part_one();
    let duration = start.elapsed();
    println!("Part one: {} - {duration:?}", part_one_result);

    let start = Instant::now();
    let part_one_opt_result = part_one_opt();
    let duration = start.elapsed();
    println!("Part one optimised: {} - {duration:?}", part_one_opt_result);

    let start = Instant::now();
    let part_two_result = part_two();
    let duration = start.elapsed();
    println!("Part two: {} - {duration:?}", part_two_result);
    
    let start = Instant::now();
    let part_two_opt_result = part_two_opt();
    let duration = start.elapsed();
    println!("Part two optimised: {} - {duration:?}", part_two_opt_result);
}


fn part_one() -> i32 {
    let mut total_score = 0;
    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            if let Ok(moves) = line {
                let (theirs, mine) = (moves.chars().nth(0), moves.chars().nth(2));
                match mine {
                    Some('X') => {
                        total_score += 1;
                        if theirs == Some('A') {
                            total_score += 3;
                        } else if theirs == Some('C') {
                            total_score += 6;
                        }
                    },
                    Some('Y') => {
                        total_score += 2;
                        if theirs == Some('A') {
                             total_score += 6;
                        } else if theirs == Some('B') {
                            total_score += 3;
                        }
                    },
                    Some('Z') => {
                        total_score += 3;
                        if theirs == Some('B') {
                            total_score += 6;
                        } else if theirs == Some('C') {
                            total_score += 3;
                        }
                    },
                    _ => panic!("No move")
                }
            }
        }
    }
    total_score
}


fn part_one_opt() -> i32 {
    let input = fs::read_to_string(String::from("input.txt")).unwrap();
    input
        .lines()
        .map(|v| v.as_bytes())
        .map(|v| match (v[0], v[2]) {
            (b'A', b'X') => 4,
            (b'A', b'Y') => 8,
            (b'A', b'Z') => 3,
            (b'B', b'X') => 1,
            (b'B', b'Y') => 5,
            (b'B', b'Z') => 9,
            (b'C', b'X') => 7,
            (b'C', b'Y') => 2,
            (b'C', b'Z') => 6,
            _ => panic!("Not valid input")
        })
        .sum()
}

fn part_two() -> i32 {
    let mut total_score = 0;
    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            if let Ok(moves) = line {
                let (theirs, mine) = (moves.chars().nth(0), moves.chars().nth(2));
                match mine {
                    Some('X') => {
                        if theirs == Some('A') {
                            total_score += 3;
                        } else if theirs == Some('B') {
                            total_score += 1
                        } else if theirs == Some('C') {
                            total_score += 2;
                        }
                    },
                    Some('Y') => {
                        total_score += 3;
                        if theirs == Some('A') {
                            total_score += 1;
                        } else if theirs == Some('B') {
                            total_score += 2;
                        } else if theirs == Some('C') {
                            total_score += 3;
                        }
                    },
                    Some('Z') => {
                        total_score += 6;
                        if theirs == Some('A') {
                            total_score += 2;
                        } else if theirs == Some('B') {
                            total_score += 3;
                        } else if theirs == Some('C') {
                            total_score += 1;
                        }
                    },
                    _ => panic!("No move")
                }
            }
        }
    }
    total_score
}

fn part_two_opt() -> i32 {
    let input = fs::read_to_string(String::from("input.txt")).unwrap();
    input
        .lines()
        .map(|v| v.as_bytes())
        .map(|v| match (v[0], v[2]) {
            (b'A', b'X') => 3,
            (b'A', b'Y') => 4,
            (b'A', b'Z') => 8,
            (b'B', b'X') => 1,
            (b'B', b'Y') => 5,
            (b'B', b'Z') => 9,
            (b'C', b'X') => 2,
            (b'C', b'Y') => 6,
            (b'C', b'Z') => 7,
            _ => panic!("Not valid input")
        })
        .sum()
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}