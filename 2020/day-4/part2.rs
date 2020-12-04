use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;
extern crate regex;

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
            last.push(' ');
          }
        }
      }
    }
  }

  let cnt = passports_data.iter().fold(0, |acc, passport| if passport_valid(passport) { acc + 1 } else { acc } );

  println!("{}", cnt);
}

fn passport_valid(passport: &str) -> bool {
  return byr_valid(passport) &&
         iyr_valid(passport) &&
         eyr_valid(passport) &&
         hgt_valid(passport) &&
         hcl_valid(passport) &&
         ecl_valid(passport) &&
         pid_valid(passport)
}

fn byr_valid(passport: &str) -> bool {
  let re = Regex::new(r"byr:(\d{4}) ").unwrap(); // byr (Birth Year) - four digits; at least 1920 and at most 2002.

  if let Some(cap) = re.captures(passport) {
    let val: u32 = (&cap[1]).to_string().parse().unwrap();
    if val >= 1920 && val <= 2002 { return true }
  }
  return false
}

fn iyr_valid(passport: &str) -> bool {
  let re = Regex::new(r"iyr:(\d{4}) ").unwrap(); // iyr (Issue Year) - four digits; at least 2010 and at most 2020.

  if let Some(cap) = re.captures(passport) {
    let val: u32 = (&cap[1]).to_string().parse().unwrap();
    if val >= 2010 && val <= 2020 { return true }
  }
  return false
}

fn eyr_valid(passport: &str) -> bool {
  let re = Regex::new(r"eyr:(\d{4}) ").unwrap(); // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.

  if let Some(cap) = re.captures(passport) {
    let val: u32 = (&cap[1]).to_string().parse().unwrap();
    if val >= 2020 && val <= 2030 { return true }
  }
  return false
}

fn hgt_valid(passport: &str) -> bool {
  let re = Regex::new(r"hgt:(\d{2,3})(\S{2}) ").unwrap(); // hgt (Height) - a number followed by either cm or in:
  // If cm, the number must be at least 150 and at most 193.
  // If in, the number must be at least 59 and at most 76.

  if let Some(cap) = re.captures(passport) {
    let val: u32 = (&cap[1]).to_string().parse().unwrap();
    let unit = (&cap[2]).to_string();
    if unit == "cm" && (val < 150 || val > 193) { return false }
    if unit == "in" && (val < 59 || val > 76) { return false }
  }
  else { return false }
  return true
}

fn hcl_valid(passport: &str) -> bool {
  let re = Regex::new(r"hcl:#([0-9a-f]{6}) ").unwrap(); // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
  return re.is_match(passport)
}

fn ecl_valid(passport: &str) -> bool {
  let re = Regex::new(r"ecl:(amb|blu|brn|gry|grn|hzl|oth) ").unwrap(); // ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
  return re.is_match(passport)
}

fn pid_valid(passport: &str) -> bool {
  let re = Regex::new(r"pid:(\d{9}) ").unwrap(); // pid (Passport ID) - a nine-digit number, including leading zeroes.  
  return re.is_match(passport)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
  let file = File::open(filename)?;
  Ok(io::BufReader::new(file).lines())
}