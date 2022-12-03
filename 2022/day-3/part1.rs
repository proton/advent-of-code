use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;

pub fn main() {
  if let Ok(lines) = read_lines("input.txt") {
    let mut priorities = Vec::new();
    
    for line in lines {
      if let Ok(line) = line {
        let chars = line.as_bytes();
        let left  = &chars[0..line.len()/2];
        let right = &chars[line.len()/2..line.len()];

        let set_a: HashSet<_> = left.into_iter().collect();
        let set_b: HashSet<_> = right.into_iter().collect();
        
        let same = set_a.intersection(&set_b).last().unwrap();

        // println!("{}", priority(**same));
        priorities.push(priority(**same) as u32);
      }
    }
    
    let sum: u32 = priorities.iter().sum();
    println!("{}", sum);
  }
}

fn priority(char: u8) -> u8 {
  return if char > 96 { char - 96 } else { char - 38 };
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
  let file = File::open(filename)?;
  Ok(io::BufReader::new(file).lines())
}