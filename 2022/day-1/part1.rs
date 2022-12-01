use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn main() {
  if let Ok(lines) = read_lines("input.txt") {
    let mut vec = Vec::<i32>::new();
    vec.push(0);

    for line in lines {
      if let Ok(line) = line {
        if line.len() > 0 {
          let calories = line.parse::<i32>().unwrap();
          *vec.last_mut().unwrap() += calories;
        } else {
          vec.push(0);
        }
      }
    }
    let most_calories = vec.iter().max().unwrap();
    println!("{}", most_calories);
  }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
  let file = File::open(filename)?;
  Ok(io::BufReader::new(file).lines())
}