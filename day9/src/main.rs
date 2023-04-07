use std::fs;
use std::collections::HashSet;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let input = fs::read_to_string("input.txt").unwrap();
    let duration = start.elapsed();
    let part_one_result = part_one(&input);
    println!("Part one: {} - {:?}", part_one_result, duration);

    let start = Instant::now();
    let part_two_result = part_two(&input);
    let duration = start.elapsed();
    println!("Part two: {} - {:?}", part_two_result, duration);
}

fn part_one(input: &str) -> usize {
    let mut head_position = (0, 0);
    let mut tail_position = (0, 0);
    let mut unique_tail_positions = HashSet::new();
    unique_tail_positions.insert((0, 0));

    let instructions: Vec<(char, usize)> = input
        .lines()
        .map(|v| v.split_ascii_whitespace().collect::<Vec<&str>>())
        .map(|v| (v[0].chars().next().unwrap(), v[1].parse().unwrap()))
        .collect();

    for (direction, steps) in instructions {
        match direction {
            'U' => {
                for _ in 0..steps {
                    head_position.1 += 1;
                    if head_position.1 > tail_position.1 + 1 {
                        tail_position.0 = head_position.0;
                        tail_position.1 = head_position.1 - 1;
                    }
                    unique_tail_positions.insert(tail_position);
                }
            },
            'D' => {
                for _ in 0..steps {
                    head_position.1 -= 1;
                    if head_position.1 < tail_position.1 - 1 {
                        tail_position.0 = head_position.0;
                        tail_position.1 = head_position.1 + 1;
                    }
                    unique_tail_positions.insert(tail_position);
                }
            },
            'L' => {
                for _ in 0..steps {
                    head_position.0 -= 1;
                    if head_position.0 < tail_position.0 - 1 {
                        tail_position.0 = head_position.0 + 1;
                        tail_position.1 = head_position.1;
                    }
                    unique_tail_positions.insert(tail_position);
                }
            },
            'R' => {
                for _ in 0..steps {
                    head_position.0 += 1;
                    if head_position.0 > tail_position.0 + 1 {
                        tail_position.0 = head_position.0 - 1;
                        tail_position.1 = head_position.1;
                    }
                    unique_tail_positions.insert(tail_position);
                }
            }
            _ => ()
        }
    }

    unique_tail_positions.len()
}

fn part_two(input: &str) -> usize {
    let mut rope: Vec<(i32, i32)> = vec![(0, 0); 10];
    let mut unique_tail_positions = HashSet::new();
    unique_tail_positions.insert((0, 0));

    let instructions: Vec<((i32, i32), i32)> = input
        .lines()
        .map(|v| v.split_ascii_whitespace().collect::<Vec<&str>>())
        .map(|v| (v[0].chars().next().unwrap(), v[1].parse().unwrap()))
        .map(|(direction, steps)| match direction {
            'U' => ((0, 1), steps),
            'D' => ((0, -1), steps),
            'L' => ((-1, 0), steps),
            'R' => ((1, 0), steps),
            _ => panic!()
        }).collect();

    for ((dx, dy), steps) in instructions {
        for _ in 0..steps {
            rope[0].0 += dx;
            rope[0].1 += dy;

            for i in 1..rope.len() {
                let ddx = rope[i - 1].0 - rope[i].0;
                let ddy = rope[i - 1].1 - rope[i].1;
                if ddx.abs() > 1 || ddy.abs() > 1 {
                    rope[i].0 += ddx.signum();
                    rope[i].1 += ddy.signum();
                }
            }
            
            unique_tail_positions.insert(*rope.last().unwrap());
        }
    }

    unique_tail_positions.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = fs::read_to_string("example.txt").unwrap();
        assert_eq!(part_one(&input), 13);
        assert_eq!(part_two(&input), 1);
    }

    #[test]
    fn larger_example() {
        let input = fs::read_to_string("larger_example.txt").unwrap();
        assert_eq!(part_two(&input), 36);
    }
}