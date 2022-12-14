use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Clone)]
enum Packet {
  Number(usize),
  Array(Vec<Packet>),
}

pub fn main() {
  if let Ok(lines) = read_lines("input.txt") {
    let mut pairs = Vec::new();

    let mut pair = Vec::new();
    for line in lines {
      if let Ok(line) = line {
        if line.len() == 0 {
            continue;
        }  
        
        pair.push(line);
        if pair.len() == 2 {
            pairs.push(pair);
            pair = Vec::new();
        }
      }
    }
    
    let mut good_pairs = Vec::new();
    
    for (id, pair) in pairs.iter().enumerate() {
        if is_good(&pair) {
            good_pairs.push(id + 1);
        }
    }
    
    // println!("{:?}", good_pairs);
    
    let sum: usize = good_pairs.iter().sum();
    println!("{}", sum);
  }
}

fn is_good(pair: &Vec<String>) -> bool {
    let packet1 = parse_line(&pair[0]);
    let packet2 = parse_line(&pair[1]);
    
    return compare(packet1, packet2);
}

fn compare(packet1: Packet, packet2: Packet) -> bool {
    if matches!(packet1, Packet::Number(_)) && matches!(packet2, Packet::Number(_)) {
        let n1 = if let Packet::Number(n1) = packet1 { n1 } else { 0 };
        let n2 = if let Packet::Number(n2) = packet2 { n2 } else { 0 };
        return n1 <= n2;
    }
    if matches!(packet1, Packet::Array(_)) && matches!(packet2, Packet::Array(_)) {
        return compare_arrays(packet1, packet2);
    }
    if matches!(packet1, Packet::Number(_)) && matches!(packet2, Packet::Array(_)) {
        return compare_number_with_array(packet1, packet2);
    }
    if matches!(packet1, Packet::Array(_)) && matches!(packet2, Packet::Number(_)) {
        return compare_array_with_number(packet1, packet2);
    }
    return false;
}

fn compare_number_with_array(packet1: Packet, packet2: Packet) -> bool {
    let a2 = if let Packet::Array(a2) = packet2 { a2 } else { vec![Packet::Number(1)] };
    for p in a2 {
        if !compare(packet1.clone(), p) {
            return false;
        }
    }
    return true;
}

fn compare_array_with_number(packet1: Packet, packet2: Packet) -> bool {
    let a1 = if let Packet::Array(a1) = packet1 { a1 } else { vec![Packet::Number(1)] };
    for p in a1 {
        if !compare(p, packet2.clone()) {
            return false;
        }
    }
    return true;
}

fn compare_arrays(packet1: Packet, packet2: Packet) -> bool {
    let a1 = if let Packet::Array(a1) = packet1 { a1 } else { vec![Packet::Number(1)] };
    let a2 = if let Packet::Array(a2) = packet2 { a2 } else { vec![Packet::Number(1)] };
    if a1.len() > a2.len() {
        return false;
    }
    for i in 0..a1.len() {
        let e1 = a1[i].clone();
        let e2 = a2[i].clone();
        if !compare(e1, e2) {
            return false;
        }
    }
    return true;
}

fn parse_line(line: &String) -> Packet {
    let mut arr = Vec::new();
    let mut start = 1;
    let end = line.len();
    while start < end - 1 {
        let e = retrieve_element(line.get(start..end).unwrap().to_string());
        start += e.0 + 1;
        arr.push(e.1);
    }
    return Packet::Array(arr);
}

fn retrieve_element(line: String) -> (usize, Packet) {
    let mut comma = find_index(&line, ',');
    let start_bracket = find_index(&line, '[');
    
    if comma == start_bracket {
        comma = line.len() - 1;
    }
    
    if comma < start_bracket {
        let n = line.get(0..comma).unwrap().parse().unwrap();
        return (comma, Packet::Number(n));
    }
    let end_bracket = find_end_bracket(&line) + 1;
    let packet = parse_line(&line.get(start_bracket..end_bracket).unwrap().to_string());
    return (end_bracket, packet);
}

fn find_end_bracket(line: &String) -> usize {
    let mut cnt = 0;
    for (i, char) in line.chars().enumerate() {
        if char == '[' { cnt += 1; }
        if char == ']' { cnt -= 1; }
        if cnt == 0 {
            return i;
        }
    }
    return 0;
}

fn find_index(str: &String, char: char) -> usize {
    if let Some(x) = str.find(char) {
        return x;
    }
    return usize::MAX;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
  let file = File::open(filename)?;
  Ok(io::BufReader::new(file).lines())
}
