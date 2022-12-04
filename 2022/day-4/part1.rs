use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn main() {
  if let Ok(lines) = read_lines("input.txt") {
    let mut pairs_count = 0;
    
    for line in lines {
      if let Ok(line) = line {
        let groups: Vec<Vec<u32>> = line.split(",").map(|s| string_to_range(s) ).collect();

        let x1 = groups[0][0];
        let x2 = groups[0][1];
        let y1 = groups[1][0];
        let y2 = groups[1][1];

        // let is_covered = (x1 <= y1 && x2 >= y2) || (y1 <= x1 && y2 >= x2);
        println!("{} {}", line, is_covered);
        if is_covered {
          pairs_count += 1;
        }
      }
    }
    
    println!("{}", pairs_count);
  }
}

fn string_to_range(string: &str) -> Vec<u32> {
  return string.split("-").map(|s| s.parse().unwrap() ).collect();
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
  let file = File::open(filename)?;
  Ok(io::BufReader::new(file).lines())
}