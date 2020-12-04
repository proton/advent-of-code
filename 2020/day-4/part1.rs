use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
  let filepath = "input.txt";
  let mut passports_data = vec!["".to_string()];

  if let Ok(lines) = read_lines(filepath) {
    for line in lines {
      if let Ok(l) = line {
        if l.len() == 0 {
          passports_data.push("".to_string())
        } else {
          if let Some(last) = passports_data.last_mut() {
            last.push_str(&(" ".to_string() + &l));
          }
        }
      }
    }
  }

  let cnt = passports_data.iter().fold(0, |acc, passport| if passport_valid(passport) { acc + 1 } else { acc } );

  println!("{}", cnt);
}

fn passport_valid(passport: &str) -> bool {
  return passport.contains("byr:") &&
         passport.contains("iyr:") &&
         passport.contains("eyr:") &&
         passport.contains("hgt:") &&
         passport.contains("hcl:") &&
         passport.contains("ecl:") &&
         passport.contains("pid:")
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
  let file = File::open(filename)?;
  Ok(io::BufReader::new(file).lines())
}
