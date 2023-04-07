use std::fs;

fn main() {
    let part_one_result = part_one();
    println!("Part one: {}", part_one_result);

    let part_two_result = part_two();
    println!("Part two: {}", part_two_result);
}

fn part_one() -> usize {
    let input = fs::read_to_string("input.txt").unwrap();
    let grid: Vec<Vec<char>> = input.lines().map(|v| v.chars().collect()).collect();
    let h = grid.len();
    let w = grid[0].len();
    let mut count = h + h + w + w - 4;
    
    for i in 1..(h - 1) {
        for j in 1..(w - 1) {
            let v = grid[i][j];
            if (0..i).all(|x| grid[x][j] < v) ||
                ((i + 1)..h).all(|x| grid[x][j] < v) ||
                (0..j).all(|x| grid[i][x] < v) ||
                ((j + 1)..w).all(|x| grid[i][x] < v) {
                count += 1;
            }

        }
    }

    count
}

fn part_two() -> usize {
    let input = fs::read_to_string("input.txt").unwrap();
    let grid: Vec<Vec<char>> = input.lines().map(|v| v.chars().collect()).collect();
    let h = grid.len();
    let w = grid[0].len();
    let mut scenic_record = 0;

    for i in 1..(h - 1) {
        for j in 1..(w - 1) {
            let v = grid[i][j];
            let up = match (0..i).rev().find(|&c| grid[c][j] >= v) {
                None => i,
                Some(x) => i - x
            };
            let down = match ((i + 1)..h).find(|&c| grid[c][j] >= v) {
                None => h - i - 1,
                Some(x) => x - i
            };
            let left = match (0..j).rev().find(|&c| grid[i][c] >= v) {
                None => j,
                Some(x) => j - x
            };
            let right = match ((j + 1)..w).find(|&c| grid[i][c] >= v) {
                None => w - j - 1,
                Some(x) => x - j
            };

            let scenic_score = up * down * left * right;
            if scenic_score > scenic_record {
                scenic_record = scenic_score;
            }
        }
    }

    scenic_record
}
