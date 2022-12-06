use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;

pub fn main() {
  if let Ok(lines) = read_lines("input.txt") {
    let mut groups = Vec::new();
    let mut priorities = Vec::new();
    
    for (i, line) in lines.enumerate() {
      if let Ok(line) = line {
        if i % 3 == 0 {
          groups = Vec::new();
        }
        groups.push(line);
        if i % 3 == 2 {
          let set_a: HashSet<_> = groups[0].bytes().into_iter().collect();
          let set_b: HashSet<_> = groups[1].bytes().into_iter().collect();
          let set_c: HashSet<_> = groups[2].bytes().into_iter().collect();

          let iter1: HashSet<_> = set_a.intersection(&set_b).copied().collect();
          let same = iter1.intersection(&set_c).last().unwrap();

          priorities.push(priority(*same) as u32);
        }
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