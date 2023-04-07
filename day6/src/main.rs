use std::fs;

fn main() {
    let part_one_result = part_one();
    println!("Part one: {}", part_one_result);

    let part_two_result = part_two();
    println!("Part two: {}", part_two_result);
}

fn part_one() -> i32 {
    let input = fs::read_to_string("input.txt").unwrap();
    let chars = input.as_bytes();
    let mut window = chars.windows(4);

    let mut index = 3;
    'windowing: while let Some(w) = window.next() {
        index += 1;
        let mut map: u32 = 0;
        for c in w {
            let index = c - b'a';
            if map & (1 << index) != 0 {
                continue 'windowing;
            }
            map |= 1 << index;
        }

        break 'windowing;
    }

    index
}

fn part_two() -> i32 {
    let input = fs::read_to_string("input.txt").unwrap();
    let chars = input.as_bytes();
    let mut window = chars.windows(14);

    let mut index = 13;
    'windowing: while let Some(w) = window.next() {
        index += 1;
        let mut map: u32 = 0;
        for c in w {
            let index = c - b'a';
            if map & (1 << index) != 0 {
                continue 'windowing;
            }
            map |= 1 << index;
        }

        break 'windowing;
    }

    index
}