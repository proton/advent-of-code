use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

pub fn main() {
  let mut scores = HashMap::new();
  scores.insert('A', 1);
  scores.insert('B', 2);
  scores.insert('C', 3);
  scores.insert('X', 1);
  scores.insert('Y', 2);
  scores.insert('Z', 3);

  let mut total_score = 0;

  if let Ok(lines) = read_lines("input.txt") {
    for line in lines {
      if let Ok(line) = line {
        let s1 = *scores.get(&line.chars().nth(0).unwrap()).unwrap();
        let s2 = *scores.get(&line.chars().nth(2).unwrap()).unwrap();
        // println!("{}", s2 + score(s1, s2));
        total_score += s2 + score(s1, s2);
      }
    }
    
    println!("{}", total_score);
  }
}

fn score(s1: i32, s2: i32) -> i32 {
  match s2 - s1 {
    1 => return 6,
    0 => return 3,
    -2=> return 6,
    _ => return 0, 
  }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
  let file = File::open(filename)?;
  Ok(io::BufReader::new(file).lines())
}