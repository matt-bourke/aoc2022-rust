use std::{fs, time::Instant, ops::RangeInclusive};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let start = Instant::now();
    let part_one_result = part_one(&input, 2000000);
    let duration = start.elapsed();
    println!("Part one: {} - {:?}", part_one_result, duration);

    let start = Instant::now();
    let part_two_result = part_two(&input, 4000000);
    let duration = start.elapsed();
    println!("Part two: {} - {:?}", part_two_result, duration);
}

fn parse_input(input: &str, y_level: i32) -> (Vec<(i32, i32, i32)>, i32) {
    let mut beacons_inline = vec![];
    let sensors = input
        .lines()
        .map(|line| line.chars().filter(|c| c.is_digit(10) || c.is_ascii_whitespace() || c == &'-').collect::<String>())
        .map(|s| {
            let nums = s.split_ascii_whitespace().collect::<Vec<&str>>();
            let x: i32 = nums[0].parse().unwrap();
            let y: i32 = nums[1].parse().unwrap();
            let bx: i32 = nums[2].parse().unwrap();
            let by: i32 = nums[3].parse().unwrap();

            if by == y_level && !beacons_inline.contains(&bx) { beacons_inline.push(bx) };

            let range = (bx - x).abs() + (by - y).abs();
            (x, y, range)
        })
        .filter(|sensor| sensor.2 >= (y_level - sensor.1).abs())
        .collect();

    (sensors, beacons_inline.len() as i32)
}

fn parse_inputs(input: &str) -> (Vec<(i32, i32, i32)>, Vec<(i32, i32)>) {
    let mut beacons = vec![];
    let sensors = input
        .lines()
        .map(|line| line.chars().filter(|c| c.is_digit(10) || c.is_ascii_whitespace() || c == &'-').collect::<String>())
        .map(|s| {
            let n = s.split_ascii_whitespace().collect::<Vec<&str>>();
            n.iter().map(|v| v.parse().unwrap()).collect::<Vec<i32>>()
        })
        .map(|v| {
            if !beacons.contains(&(v[2], v[3])) { beacons.push((v[2], v[3])) }
            let range = (v[2] - v[0]).abs() + (v[3] - v[1]).abs();
            (v[0], v[1], range)
        })
        .collect::<Vec<(i32, i32, i32)>>();
    
    (sensors, beacons)
}

fn find_ranges(sensors: &Vec<(i32, i32, i32)>, y_level: i32) -> Vec<RangeInclusive<i32>> {
    let mut ranges = vec![];

    for sensor in sensors.iter() {
        let overflow = sensor.2 - (sensor.1 - y_level).abs();
        let range = (sensor.0 - overflow)..=(sensor.0 + overflow);
        ranges.push(range);
    }

    ranges
}

fn filter_and_find_ranges(sensors: &Vec<(i32, i32, i32)>, y_level: i32) -> Vec<RangeInclusive<i32>> {
    let mut ranges = vec![];

    for sensor in sensors.iter() {
        let overflow = sensor.2 - (sensor.1 - y_level).abs();
        if overflow < 0 {
            continue;
        }

        ranges.push((sensor.0 - overflow)..=(sensor.0 + overflow));
    }

    ranges
}

fn merge_ranges(ranges: Vec<RangeInclusive<i32>>) -> Vec<RangeInclusive<i32>> {
    let mut merged = vec![];
    
    let mut a = ranges[0].clone();
    for b in &ranges[1..] {
        if *a.end() < *b.start() - 1 {
            merged.push(a.clone());
            a = b.clone();
        } else if *a.end() < *b.end() {
            a = *a.start()..=*b.end();
        } 
    }

    merged.push(a.clone());
    merged
}

fn part_one(input: &str, y_level: i32) -> i32 {
    let (sensors, num_beacons_inline) = parse_input(&input, y_level);
    let mut ranges = find_ranges(&sensors, y_level);

    ranges.sort_by_key(|v| *v.start());

    let merged_ranges = merge_ranges(ranges);
    merged_ranges
        .iter()
        .map(|v| *v.end() - *v.start() + 1)
        .sum::<i32>() - num_beacons_inline
}

fn part_two(input: &str, d_max: i32) -> usize {
    let (sensors, _beacons) = parse_inputs(&input);
    for y in (0..=d_max).rev() {
        let mut ranges = filter_and_find_ranges(&sensors, y);
        // println!("{y}");
        ranges.sort_by_key(|v| *v.start());
        let merged_ranges = merge_ranges(ranges);
        let clamped_ranges = merged_ranges.iter().map(|r| {
            if *r.start() < 0 && *r.end() > d_max {
                0..=d_max
            } else if *r.start() < 0 {
                0..=*r.end()
            } else if *r.end() > d_max {
                *r.start()..=d_max
            } else {
                *r.start()..=*r.end()
            }
        }).collect::<Vec<RangeInclusive<i32>>>();

        if clamped_ranges.len() > 1 {
            let x = *clamped_ranges[0].end() as usize + 1;
            let y = y as usize;
            return (x * 4000000) + y;
        }
    }

    0 as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let input = fs::read_to_string("example.txt").unwrap();
        assert_eq!(part_one(&input, 10), 26);
    }

    #[test]
    fn example_two() {
        let input = fs::read_to_string("example.txt").unwrap();
        assert_eq!(part_two(&input, 20), 56000011);
    }
}