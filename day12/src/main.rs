use std::{fs, collections::{VecDeque, HashSet}, time::Instant};

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

fn parse_input(input: &str) -> ((i32, i32), (i32, i32), Vec<Vec<u8>>) {
    let mut row = 0;
    let mut col = 0;
    let mut start = (0, 0);
    let mut end = (0, 0);
    let grid = input
        .lines()
        .map(|v| v.chars().collect())
        .map(|v: Vec<char>| {
            row += 1;
            v.iter().map(|&c| {
                col = (col + 1) % v.len() as i32;
                match c {
                    'S' => {
                        start = (col - 1, row - 1);
                        0
                    },
                    'E' => {
                        end = (col - 1, row - 1);
                        25
                    },
                    'a'..='z' => {
                        (c as u8) - (b'a' as u8)
                    },
                    _ => panic!()
                }
            })
            .collect()
        })
        .collect();

    (start, end, grid)
}

fn part_one(input: &str) -> i32 {
    let (start, end, grid) = parse_input(&input);
    let height: i32 = grid.len() as i32;
    let width: i32 = grid[0].len() as i32;

    let mut open_set: VecDeque<(i32, (i32, i32))> = VecDeque::new();
    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    open_set.push_back((0, start));

    while let Some((steps, p)) = open_set.pop_front() {
        if p == end {
            return steps;
        }

        if !visited.insert(p) {
            continue;
        }

        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let (nx, ny) = (p.0 as i32 + dx, p.1 as i32 + dy);

            if nx < 0 || nx >= width || ny < 0 || ny >= height {
                continue;
            }
            
            let np = (nx, ny);
            if visited.contains(&np) {
                continue;
            }

            let dh = grid[np.1 as usize][np.0 as usize] as i32 - grid[p.1 as usize][p.0 as usize]as i32;
            if dh <= 1 {
                open_set.push_back((steps + 1, np));
            }
        }
    }

    -1
}

fn part_two(input: &str) -> i32 {
    let (_, end, grid) = parse_input(&input);
    let height = grid.len() as i32;
    let width = grid[0].len() as i32;

    let mut open_set: VecDeque<(i32, (i32, i32))> = VecDeque::new();
    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    open_set.push_back((0, end));
    
    while let Some((steps, p)) = open_set.pop_front() {
        if grid[p.1 as usize][p.0 as usize] == 0 {
            return steps;
        }

        if !visited.insert(p) {
            continue;
        }

        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let (nx, ny) = (p.0 as i32 + dx, p.1 as i32 + dy);

            if nx < 0 || nx >= width || ny < 0 || ny >= height {
                continue;
            }
            
            let np = (nx, ny);
            if visited.contains(&np) {
                continue;
            }

            let dh = grid[np.1 as usize][np.0 as usize] as i32 - grid[p.1 as usize][p.0 as usize]as i32;
            if dh >= -1 {
                open_set.push_back((steps + 1, np));
            }
        }
    }
    
    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = fs::read_to_string("example.txt").unwrap();
        assert_eq!(part_one(&input), 31);
        assert_eq!(part_two(&input), 29);
    }
}