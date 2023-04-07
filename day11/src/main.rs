use std::fs;
use std::collections::VecDeque;
use std::time::Instant;

struct Monkey {
    items: VecDeque<i64>,
    operation: (char, Option<i64>),
    test_value: i64,
    throw_to: (usize, usize)
}

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

fn parse_input(input: &str) -> Vec<Monkey> {
    input.lines().collect::<Vec<&str>>()[..]
        .chunks(7)
        .map(|v| Monkey {
            items: v[1][18..]
                .split(',')
                .map(|x| x.trim().parse().unwrap())
                .collect(),
            operation: (v[2][23..24].chars().next().unwrap(), v[2][25..].trim().parse().ok()),
            test_value: v[3][21..].trim().parse().unwrap(),
            throw_to: (v[4][29..].trim().parse().unwrap(), v[5][30..].trim().parse().unwrap())
        })
        .collect()
}

fn part_one(input: &str) -> i64 {
    let mut monkeys = parse_input(&input);
    let mut inspections = vec![0; monkeys.len()];

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            while let Some(v) = monkeys[i].items.pop_front() {
                let mut new = match monkeys[i].operation {
                    ('+', Some(x)) => v + x,
                    ('*', Some(x)) => v * x,
                    ('+', None) => v + v,
                    ('*', None) => v * v,
                    _ => panic!()
                };

                inspections[i] += 1;
                new /= 3;

                let throw_to = if new % monkeys[i].test_value == 0 {
                    monkeys[i].throw_to.0
                } else {
                    monkeys[i].throw_to.1
                };

                monkeys[throw_to].items.push_back(new);
            }
        }
    }

    inspections.sort_unstable_by(|a, b| b.cmp(a));
    inspections[0] * inspections[1]
}

fn part_two(input: &str) -> i64 {
    let mut monkeys = parse_input(&input);
    let mut inspections = vec![0; monkeys.len()];
    let supermodulo: i64 = monkeys.iter().map(|v| v.test_value).product();

    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            while let Some(v) = monkeys[i].items.pop_front() {
                let new = match monkeys[i].operation {
                    ('+', Some(x)) => v + x,
                    ('*', Some(x)) => v * x,
                    ('+', None) => v + v,
                    ('*', None) => v * v,
                    _ => panic!()
                } % supermodulo / 1;

                inspections[i] += 1;

                let throw_to = if new % monkeys[i].test_value == 0 {
                    monkeys[i].throw_to.0
                } else {
                    monkeys[i].throw_to.1
                };

                monkeys[throw_to].items.push_back(new);
            }
        }
    }

    inspections.sort_unstable_by(|a, b| b.cmp(a));
    inspections[0] * inspections[1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = fs::read_to_string("input.txt").unwrap();
        assert_eq!(part_one(&input), 10605);
    }
}
