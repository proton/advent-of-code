use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Monkey {
  number: usize,
  starting_items: Vec<usize>,
  items: Vec<usize>,
  operation: String,
  test_divisible_by: i32,
  throw_true: usize,
  throw_false: usize,
}

impl Monkey {
    fn new() -> Monkey {
        return Monkey {
            number: 0,
            starting_items: Vec::new(),
            items: Vec::new(),
            operation: "".to_string(),
            test_divisible_by: 0,
            throw_true: 0,
            throw_false: 0,
        };
    }
}

pub fn main() {
  if let Ok(lines) = read_lines("input.txt") {
    let mut monkeys = Vec::new();

    let mut current_monkey = Monkey::new();
    for line in lines {
      if let Ok(line) = line {
        let splitted_line: Vec<_> = line.split(": ").collect();
        if line.len() == 0 {
            monkeys.push(current_monkey);
            current_monkey = Monkey::new();
        } else if line.starts_with("Monkey ") {
            current_monkey.number = line[7..8].parse().unwrap();
        } else if line.contains("Starting items") {
            current_monkey.starting_items = splitted_line[1].split(", ").map(|x| x.parse().unwrap()).collect();
            current_monkey.items = current_monkey.starting_items.clone();
        } else if line.contains("Operation") {
            current_monkey.operation = splitted_line[1].to_string();
        } else if line.contains("Test") {
            let spl: Vec<_> = splitted_line[1].split("divisible by ").collect();
            current_monkey.test_divisible_by = spl[1].parse().unwrap();
        } else if line.contains("If true") {
            let spl: Vec<_> = splitted_line[1].split("throw to monkey ").collect();
            current_monkey.throw_true = spl[1].parse().unwrap();
        } else if line.contains("If false") {
            let spl: Vec<_> = splitted_line[1].split("throw to monkey ").collect();
            current_monkey.throw_false = spl[1].parse().unwrap();
        }
      }
    }
    monkeys.push(current_monkey);
    
    println!("{}", monkeys.len());
  }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
  let file = File::open(filename)?;
  Ok(io::BufReader::new(file).lines())
}
