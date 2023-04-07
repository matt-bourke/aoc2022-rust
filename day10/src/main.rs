use std::{fs, time::Instant};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let start = Instant::now();
    let part_one_result = part_one(&input);
    let duration = start.elapsed();
    println!("Part one: {} - {:?}", part_one_result, duration);

    let start = Instant::now();
    let part_two_result = part_two(&input);
    let duration = start.elapsed();
    println!("Part two: {} - {:?}", part_two_result, duration);
}

fn part_one(input: &str) -> isize {
    let cycles_of_interest = [20, 60, 100, 140, 180, 220];
    let mut signal_total = 0;
    let mut x_value: isize = 1;
    let mut cycle = 1;
    
    for line in input.lines() {
        match line.split_ascii_whitespace().collect::<Vec<&str>>()[..] {
            ["noop"] => {
                if cycles_of_interest.contains(&cycle) {
                    let signal_strength = cycle * x_value;
                    signal_total += signal_strength;
                }
                cycle += 1;
            },
            ["addx", dx] => {
                if cycles_of_interest.contains(&cycle) {
                    let signal_strength = cycle * x_value;
                    signal_total += signal_strength;
                }

                cycle += 1;

                if cycles_of_interest.contains(&cycle) {
                    let signal_strength = cycle * x_value;
                    signal_total += signal_strength;
                }

                x_value += dx.parse::<isize>().unwrap();
                cycle += 1;
            },
            _ => ()
        }
    }

    signal_total
}

fn part_two(input: &str) -> String {
    let mut crt: Vec<char> = vec![];
    let mut cycle = 1;
    let mut x_value = 1;

    for line in input.lines() {
        match line.split_ascii_whitespace().collect::<Vec<&str>>()[..] {
            ["noop"] => {
                if ((x_value - 1)..=(x_value + 1)).contains(&(cycle - 1)) {
                    crt.push('#');
                } else {
                    crt.push('.');
                }

                cycle += 1;
                if cycle == 41 {
                    cycle = 1;
                }
            },
            ["addx", dx] => {
                if ((x_value - 1)..=(x_value + 1)).contains(&(cycle - 1)) {
                    crt.push('#');
                } else {
                    crt.push('.');
                }

                cycle += 1;
                if cycle == 41 {
                    cycle = 1;
                }

                if ((x_value - 1)..=(x_value + 1)).contains(&(cycle - 1)) {
                    crt.push('#');
                } else {
                    crt.push('.');
                }

                x_value += dx.parse::<isize>().unwrap();
                cycle += 1;
                if cycle == 41 {
                    cycle = 1;
                }
            },
            _ => ()
        }
    }

    [
        String::new(),
        crt[..]
            .chunks(40)
            .map(|v| v.iter().collect())
            .collect::<Vec<String>>()
            .join("\n")
    ].join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn example() {
        let input = fs::read_to_string("example.txt").unwrap();
        assert_eq!(part_one(&input), 13140);
    }
}

// [1, 1, 16, 16, 5, 5, 11, 11, 8, 8]