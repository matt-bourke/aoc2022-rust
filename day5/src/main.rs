use std::fs;

fn main() {
    let part_one_result = part_one();
    println!("Part one: {}", part_one_result);

    let part_two_result = part_two();
    println!("Part two: {}", part_two_result);
}

fn part_one() -> String {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut stacks : Vec<Vec<char>> = vec![];
    let mut lines = input.lines();
    'stacks: for line in lines.by_ref() {
        let mut chars = line.trim_end().chars();
        let mut n = 0;
        while let Some(c) = chars.nth(1) {
            if n >= stacks.len() {
                stacks.push(vec![]);
            }

            match c {
                'A'..='Z' => stacks[n].push(c),
                '1'..='9' => break 'stacks,
                ' ' => (),
                _ => panic!()
            }
        
            chars.next();
            chars.next();
            n += 1;
        }
    }

    for stack in stacks.iter_mut() {
        stack.reverse();
    }

    lines.next();
    
    let mut instructions: Vec<(usize, usize, usize)> = vec![];
    for instruction in lines.by_ref() {
        let mut splits = instruction.split_ascii_whitespace();
       instructions.push((
            splits.nth(1).unwrap().parse().unwrap(),
            splits.nth(1).unwrap().parse().unwrap(),
            splits.nth(1).unwrap().parse().unwrap()
       ));
    }

    for (index, &(moves, from, to)) in instructions.iter().enumerate() {
        for _ in 0..moves {
            let container = stacks[from - 1].pop().expect(format!("No elements left in stack {from}. Falied on operation {index}.").as_str());
            stacks[to - 1].push(container);
        }
    }

    stacks.iter()
        .map(|v| v.last().unwrap())
        .collect()
}

fn part_two() -> String {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut stacks : Vec<Vec<char>> = vec![];
    let mut lines = input.lines();
    'stacks: for line in lines.by_ref() {
        let mut chars = line.trim_end().chars();
        let mut n = 0;
        while let Some(c) = chars.nth(1) {
            if n >= stacks.len() {
                stacks.push(vec![]);
            }

            match c {
                'A'..='Z' => stacks[n].push(c),
                '1'..='9' => break 'stacks,
                ' ' => (),
                _ => panic!()
            }
        
            chars.next();
            chars.next();
            n += 1;
        }
    }

    for stack in stacks.iter_mut() {
        stack.reverse();
    }

    lines.next();
    
    let mut instructions: Vec<(usize, usize, usize)> = vec![];
    for instruction in lines.by_ref() {
        let mut splits = instruction.split_ascii_whitespace();
       instructions.push((
            splits.nth(1).unwrap().parse().unwrap(),
            splits.nth(1).unwrap().parse().unwrap(),
            splits.nth(1).unwrap().parse().unwrap()
       ));
    }

    for (index, &(moves, from, to)) in instructions.iter().enumerate() {
        let mut flipper: Vec<char> = vec![];
        for _ in 0..moves {
            let container = stacks[from - 1].pop().expect(format!("No elements left in stack {from}. Falied on operation {index}.").as_str());
            flipper.push(container);
        }
        for _ in 0..moves {
            let container = flipper.pop().unwrap();
            stacks[to - 1].push(container);
        }
    }

    stacks.iter()
        .map(|v| v.last().unwrap())
        .collect()
}