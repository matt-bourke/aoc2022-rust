use std::fs;

fn main() {
    let part_one_result = part_one();
    println!("part one: {}", part_one_result);
     
    let part_two_result = part_two();
    println!("part two: {}", part_two_result);
}

fn part_one() -> usize {
    let input = fs::read_to_string("input.txt").unwrap();
    input
        .lines()
        .map(|v| {
            v.split(&['-', ',']).map(|v| v.parse().unwrap()).collect::<Vec<i32>>()
        })
        .filter(|v| (v[0] >= v[2] && v[1] <= v[3]) || (v[2] >= v[0] && v[3] <= v[1]))
        .count()
}

fn part_two() -> usize {
    let input = fs::read_to_string("input.txt").unwrap();
    input
        .lines()
        .map(|v| {
            v.split(&['-', ',']).map(|v| v.parse().unwrap()).collect::<Vec<i32>>()
        })
        .filter(|v| (v[0] <= v[2] && v[1] >= v[3]) || (v[3] <= v[0] && v[3] >= v[3]))
        .count()
}