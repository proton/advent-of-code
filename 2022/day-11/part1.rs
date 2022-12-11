use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Monkey {
  number: usize,
  items: Vec<i64>,
  operation: String,
  operation_self: bool,
  operation_value: i64,
  test_divisible_by: i64,
  throw_true: usize,
  throw_false: usize,
  inspect_cnt: usize,
}

impl Monkey {
    fn new() -> Monkey {
        return Monkey {
            number: 0,
            items: Vec::new(),
            operation: " ".to_string(),
            operation_self: false,
            operation_value: 0,
            test_divisible_by: 0,
            throw_true: 0,
            throw_false: 0,
            inspect_cnt: 0,
        };
    }
}

pub fn main() {
  if let Ok(lines) = read_lines("input.txt") {
    let mut monkeys = Vec::new();

    let mut monkey = Monkey::new();
    for line in lines {
      if let Ok(line) = line {
        let splitted_line: Vec<_> = line.split(": ").collect();
        if line.len() == 0 {
            monkeys.push(monkey);
            monkey = Monkey::new();
        } else if line.starts_with("Monkey ") {
            monkey.number = line[7..8].parse().unwrap();
        } else if line.contains("Starting items") {
            monkey.items = splitted_line[1].split(", ").map(|x| x.parse().unwrap()).collect();
        } else if line.contains("Operation") {
            let spl: Vec<_> = splitted_line[1].split(" ").collect();
            monkey.operation = spl[3].to_string();
            if spl[4] == "old" {
                monkey.operation_self = true;
            } else {
                monkey.operation_value = spl[4].parse().unwrap();
            }
        } else if line.contains("Test") {
            let spl: Vec<_> = splitted_line[1].split("divisible by ").collect();
            monkey.test_divisible_by = spl[1].parse().unwrap();
        } else if line.contains("If true") {
            let spl: Vec<_> = splitted_line[1].split("throw to monkey ").collect();
            monkey.throw_true = spl[1].parse().unwrap();
        } else if line.contains("If false") {
            let spl: Vec<_> = splitted_line[1].split("throw to monkey ").collect();
            monkey.throw_false = spl[1].parse().unwrap();
        }
      }
    }
    monkeys.push(monkey);
    
    for _round in 0..20 {
        for i in 0..monkeys.len() { 
            // let i = round % monkeys.len();
            
            let items = monkeys[i].items.clone();
            for item in items {
                let value = operation_result(&monkeys[i], item) / 3;
                let next_monkey_i = next_monkey_i(&monkeys[i], value);
                monkeys[i].inspect_cnt += 1;
                monkeys[next_monkey_i].items.push(value);
                // println!("Item with worry level {} is thrown to monkey {}.", value, next_monkey_i);
            }
            monkeys[i].items = Vec::new();
        }
        
        // for monkey in &monkeys {
        //     println!("Monkey {}: {:?}", monkey.number, monkey.items);
        // }
    }
    
    for monkey in &monkeys {
        println!("Monkey {} inspected items {} times.", monkey.number, monkey.inspect_cnt);
    }
    
    let mut activeness: Vec<_> = monkeys.iter().map(|monkey| monkey.inspect_cnt).collect();
    activeness.sort();
    activeness.reverse();
    
    println!("{:?}", activeness[0] * activeness[1]);
  }
}

fn next_monkey_i(monkey: &Monkey, result: i64) -> usize {
    // println!("{} / {} == {}", result, monkey.test_divisible_by, result % monkey.test_divisible_by);
    if result % monkey.test_divisible_by == 0 {
        return monkey.throw_true;
    } else {
        return monkey.throw_false;
    }
}

fn operation_result(monkey: &Monkey, item: i64) -> i64 {
    let operation_value = operation_value(monkey, item);
    // println!("{}{}{}", item, monkey.operation, operation_value);
    if monkey.operation == "+" {
        return item + operation_value;
    } else { //  monkeys[i].operation == "*"
        return item * operation_value;
    }
}

fn operation_value(monkey: &Monkey, item: i64) -> i64 {
    if monkey.operation_self {
        return item;
    } else {
        return monkey.operation_value;
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
  let file = File::open(filename)?;
  Ok(io::BufReader::new(file).lines())
}
