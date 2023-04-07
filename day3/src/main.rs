use std::fs;

fn main() {
    let part_one_result = part_one();
    println!("Part one: {}", part_one_result);

    let part_two_result = part_two();
    println!("Part two: {}", part_two_result);
}

fn part_one() -> i32 {
    let input = fs::read_to_string("input.txt").unwrap();
    let lines: Vec<Vec<u8>> = input.lines()
        .map(|v| {
            v.bytes()
                .map(|v| match v {
                    b'a'..=b'z' => v - b'a' + 1,
                    b'A'..=b'Z' => v - b'A' + 27,
                    _ => 0
                })
                .collect()
        })
        .collect();

    lines
        .iter()
        .map(|v| {
            let n = v.len() / 2;
            let mut occupied = [false; 52];
            v[..n].iter().for_each(|&i| occupied[i as usize - 1] = true);
            v[n..].iter().find(|&&i| occupied[i as usize - 1]).unwrap()
        })
        .map(|&v| v as i32)
        .sum()
}

fn part_two() -> i32 {
    let input = fs::read_to_string("input.txt").unwrap();
    let lines: Vec<Vec<u8>> = input.lines()
        .map(|v| {
            v.bytes()
                .map(|v| match v {
                    b'a'..=b'z' => v - b'a' + 1,
                    b'A'..=b'Z' => v - b'A' + 27,
                    _ => 0
                })
                .collect()
        })
        .collect();

    lines
        .chunks(3)
        .map(|v| {
            let mut occupied = [(false, false); 52];
            v[0].iter().for_each(|&i| occupied[i as usize - 1].0 = true);
            v[1].iter().for_each(|&i| occupied[i as usize - 1].1 = true);
            v[2].iter().find(|&&i| occupied[i as usize - 1] == (true, true)).unwrap()
        })
        .map(|&v| v as i32)
        .sum()
}