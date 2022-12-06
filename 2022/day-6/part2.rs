use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;

pub fn main() {
  if let Ok(lines) = read_lines("input.txt") {
    for line in lines {
      if let Ok(line) = line {
        let chars: Vec<char> = line.chars().collect();
        let signal_len = 14;

        for i in 0..(chars.len()) {
          let signal  = &chars[i..(i + signal_len)].iter().collect::<HashSet<_>>();
          if signal.len() == signal_len {
            println!("{}", i + signal_len);
            break;
          }
        }
      }
    }
  }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
  let file = File::open(filename)?;
  Ok(io::BufReader::new(file).lines())
}
