use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::fmt;
use std::cmp;

#[derive(Clone)]
enum Packet {
  Number(usize),
  Array(Vec<Packet>),
}

impl fmt::Debug for Packet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Packet::Number(n) => write!(f, "{}", n),
            Packet::Array(arr) => write!(f, "{:?}", arr),
        }
    }
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
    
    return compare(packet1, packet2) <= 0;
}

fn arrairize(packet: Packet) -> Packet {
    return Packet::Array(vec![packet]);
}

fn compare(packet1: Packet, packet2: Packet) -> i32 {
    // println!("{:?} vs {:?}", packet1, packet2);
    let mut c = 0;
    if matches!(packet1, Packet::Number(_)) && matches!(packet2, Packet::Number(_)) {
        // println!("compare_numbers");
        c = compare_numbers(packet1, packet2);
    }
    else if matches!(packet1, Packet::Array(_)) && matches!(packet2, Packet::Array(_)) {
        // println!("compare_arrays");
        c = compare_arrays(packet1, packet2);
    }
    else if matches!(packet1, Packet::Number(_)) && matches!(packet2, Packet::Array(_)) {
        // println!("compare_number_with_array");
        c = compare_arrays(arrairize(packet1), packet2);
    }
    else if matches!(packet1, Packet::Array(_)) && matches!(packet2, Packet::Number(_)) {
        // println!("compare_array_with_number");
        c = compare_arrays(packet1, arrairize(packet2));
    }
    // println!("=> {}", c);
    return c;
}

fn compare_numbers(packet1: Packet, packet2: Packet) -> i32 {
    let n1 = if let Packet::Number(n1) = packet1 { n1 } else { 0 };
    let n2 = if let Packet::Number(n2) = packet2 { n2 } else { 0 };
    if n1 < n2 {
        return -1;
    }
    if n1 > n2 {
        return 1;
    }
    return 0;
}

fn compare_arrays(packet1: Packet, packet2: Packet) -> i32 {
    let a1 = if let Packet::Array(a1) = packet1 { a1 } else { vec![Packet::Number(1)] };
    let a2 = if let Packet::Array(a2) = packet2 { a2 } else { vec![Packet::Number(1)] };
    
    let m = cmp::max(a1.len(), a2.len());
    for i in 0..m {
        if i == a1.len() {
            return -1;
        }
        if i == a2.len() {
            return 1;
        }
        let e1 = a1[i].clone();
        let e2 = a2[i].clone();
        let c = compare(e1, e2);
        if c != 0 {
            return c;
        }
    }
    return 0;
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
