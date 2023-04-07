use std::{fs, time::Instant, cmp::Ordering};

#[derive(Debug, Ord, Eq, PartialEq, Clone)]
enum Packet {
    Integer(i32),
    List(Vec<Packet>)
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Packet::Integer(a), Packet::Integer(b)) => a.partial_cmp(b),
            (Packet::Integer(a), Packet::List(_)) => Packet::List(vec![Packet::Integer(*a)]).partial_cmp(other),
            (Packet::List(_), Packet::Integer(b)) => self.partial_cmp(&Packet::List(vec![Packet::Integer(*b)])),
            (Packet::List(a), Packet::List(b)) => {
                let mut a_iter = a.iter();
                let mut b_iter = b.iter();
                loop {
                    match (a_iter.next(), b_iter.next()) {
                        (None, None) => break Some(Ordering::Equal),
                        (None, Some(_)) => break Some(Ordering::Less),
                        (Some(_), None) => break Some(Ordering::Greater),
                        (Some(a), Some(b)) => {
                            match a.partial_cmp(b) {
                                Some(Ordering::Equal) => continue,
                                result => break result
                            }
                        }
                    }
                }
            }
        }
    }
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


fn parse_input(input: &str) -> Vec<(Packet, Packet)> {
    input
        .lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|v| {
            let left = v[0];
            let right = v[1];

            let left_packet = unpack(&left);
            let right_packet = unpack(&right);
    
            (left_packet, right_packet)
        })
        .collect()
}

fn unpack(packet: &str) -> Packet {
    let mut packets: Vec<Packet> = vec![];
    let data = &packet[1..packet.len() - 1];
    let mut int_string = String::new();
    let mut index_base = 0;
    for (i, d) in data.chars().enumerate() {
        if i < index_base {
            continue;
        }

        match d {
            '0'..='9' => {
                int_string.push_str(&d.to_string());
            },
            ',' => {
                if !int_string.is_empty() {
                    let n = int_string.parse().unwrap();
                    packets.push(Packet::Integer(n));
                    int_string.clear();
                }
            },
            '[' => {
                let sub = &data.chars().collect::<Vec<char>>()[i..];
                let mut brace_count = 0;
                for (index_match, cx) in sub.iter().enumerate(){
                    if cx == &'[' {
                        brace_count += 1;
                    } else if cx == &']' {
                        brace_count -= 1;
                        if brace_count == 0 {
                            let subpacket = unpack(&data[i..=i + index_match]);
                            packets.push(subpacket);
                            index_base = i + index_match;
                            break;
                        }
                    }
                }
            },
            ']' => {
                if !int_string.is_empty() {
                    let n = int_string.parse().unwrap();
                    packets.push(Packet::Integer(n));
                    int_string.clear();
                }
            }
            _ => panic!("Invalid char")
        }
    }

    if !int_string.is_empty() {
        let n = int_string.parse().unwrap();
        packets.push(Packet::Integer(n));
        int_string.clear();
    }

    Packet::List(packets)
}


fn part_one(input: &str) -> usize {
    let packet_pairs = parse_input(&input);

    packet_pairs.iter()
        .enumerate()
        .filter(|&(_, v)| v.0 < v.1)
        .map(|(i, _)| i + 1)
        .sum()
}


fn part_two(input: &str) -> usize {
    let packets_pairs = parse_input(&input);
    let mut packets = vec![];
    for (left, right) in packets_pairs {
        packets.push(left);
        packets.push(right);
    }
    
    let first_div = Packet::List(vec![Packet::List(vec![Packet::Integer(2)])]);
    let second_div = Packet::List(vec![Packet::List(vec![Packet::Integer(6)])]);
    packets.push(first_div.clone());
    packets.push(second_div.clone());

    packets.sort_unstable();

    packets.iter()
        .enumerate()
        .filter(|&(_, v)| v == &first_div || v == &second_div)
        .map(|(i, _)| i + 1)
        .product()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let input = fs::read_to_string("example.txt").unwrap();
        assert_eq!(part_one(&input), 13);
    }

    #[test]
    fn example_two() {
        let input = fs::read_to_string("example.txt").unwrap();
        assert_eq!(part_two(&input), 140)
    }
}