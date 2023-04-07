use std::fs;
use std::error::Error;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let part_one_result = part_one().expect("Error");
    let duration = start.elapsed();
    println!("Part One: {part_one_result}. Calculated in {:?}", duration);

    let start = Instant::now();
    let part_two_result = part_two().expect("Error");
    let duration = start.elapsed();
    println!("Part Two: {part_two_result}. Calculated in {:?}", duration);
}

fn part_one() -> Result<i32, Box<dyn Error>> {
    let lines = fs::read_to_string(String::from("input.txt"))?;
    let lines : Vec<&str> = lines.lines().collect();
    
    let mut record = 0;
    let mut total = 0;
    for line in lines {
        if line.is_empty() {
            if total > record {
                record = total;
            }
            total = 0;
        } else {
            total += line.parse::<i32>().unwrap();
        }
    }
    
    Ok(record)
}

fn part_two() -> Result<i32, Box<dyn Error>> {
    let lines = fs::read_to_string(String::from("input.txt")).unwrap();
    let mut sections : Vec<i32> = lines
        .split("\n\n")
        .map(|g| g.trim().lines().map(|s| s.parse().expect("I can't read")).collect::<Vec<i32>>())
        .map(|g| g.into_iter().sum())
        .collect();

    sections.sort_unstable();
    Ok(sections.into_iter().rev().take(3).sum())
}