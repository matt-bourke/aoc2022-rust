use std::{fs, time::Instant, collections::HashSet};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Point {
    x: i32,
    y: i32
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

fn parse_input(input: &str) -> (HashSet<Point>, i32) {
    let mut rock_points = HashSet::new();
    
    input
        .lines()
        .map(|v| v.split(" -> ").collect::<Vec<&str>>())
        .map(|v| v.iter().map(|vv| vv.split(",").collect::<Vec<&str>>()).collect::<Vec<_>>())
        .map(|v| v.into_iter().flatten().collect::<Vec<_>>())
        .for_each(|v| {
            let points = v.chunks(2)
                .map(|v| Point { x: v[0].parse().unwrap(), y: v[1].parse().unwrap() })
                .collect::<Vec<Point>>();
            for i in 1..points.len() {
                let start = &points[i - 1];
                let end = &points[i];

                let x_steps = (end.x - start.x).abs() + 1;
                let y_steps = (end.y - start.y).abs() + 1;
                let x_dir = (end.x - start.x).signum();
                let y_dir = (end.y - start.y).signum();

                if x_steps > 1 {
                    for i in 0..x_steps {
                        let x = start.x + (x_dir * i);
                        let y = start.y;
                        rock_points.insert(Point { x, y });
                    }
                }

                if y_steps > 1 {
                    for i in 0..y_steps {
                        let x = start.x;
                        let y = start.y + (y_dir * i);
                        rock_points.insert(Point { x, y});
                    }
                }
            }
        });

    let y_max = rock_points.iter().max_by(|a, b| (a.y).cmp(&b.y)).unwrap().y;
    (rock_points, y_max)
}

fn part_one(input: &str) -> usize {
    let (mut occupied_points, y_max) = parse_input(&input);
    let mut sand_settled = 0;
    let mut current_position = Point {
        x: 500,
        y: 0
    };

    while current_position.y < y_max {
        if !occupied_points.contains(&Point { x: current_position.x, y: current_position.y + 1 }) {
            current_position = Point { x: current_position.x, y: current_position.y + 1};
            continue
        } else if !occupied_points.contains(&Point { x: current_position.x - 1, y: current_position.y + 1 }) {
            current_position = Point { x: current_position.x - 1, y: current_position.y + 1 };
            continue;
        } else if !occupied_points.contains(&Point { x: current_position.x + 1, y: current_position.y + 1 }) {
            current_position = Point { x: current_position.x + 1, y: current_position.y + 1 };
            continue;
        } else {
            // settling
            occupied_points.insert(current_position.clone());
            sand_settled += 1;
            current_position = Point {
                x: 500,
                y: 0
            };
        }
    }

    sand_settled
}

fn part_two(input: &str) -> usize {
    let (mut occupied_points, y_max) = parse_input(&input);
    let mut sand_settled = 0;
    let floor_level = y_max + 2;
    let start_position = Point { x: 500, y: 0 };
    let mut current_position = start_position.clone();

    while !occupied_points.contains(&start_position) {
        if current_position.y != floor_level - 1 {
            if !occupied_points.contains(&Point { x: current_position.x, y: current_position.y + 1 }) {
                current_position = Point { x: current_position.x, y: current_position.y + 1};
                continue
            } else if !occupied_points.contains(&Point { x: current_position.x - 1, y: current_position.y + 1 }) {
                current_position = Point { x: current_position.x - 1, y: current_position.y + 1 };
                continue;
            } else if !occupied_points.contains(&Point { x: current_position.x + 1, y: current_position.y + 1 }) {
                current_position = Point { x: current_position.x + 1, y: current_position.y + 1 };
                continue;
            }
        }

        occupied_points.insert(current_position.clone());
        sand_settled += 1;
        current_position = Point { x: 500, y: 0 };
    }

    sand_settled
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let input = fs::read_to_string("example.txt").unwrap();
        assert_eq!(part_one(&input), 24);
    }

    #[test]
    fn example_two() {
        let input = fs::read_to_string("example.txt").unwrap();
        assert_eq!(part_two(&input), 93);
    }
}