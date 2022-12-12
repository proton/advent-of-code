use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Monkey {
  number: usize,
  items: Vec<u128>,
  operation: String,
  operation_self: bool,
  operation_value: u128,
  test_divisible_by: u128,
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
    let mut divisibles = Vec::new();
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
            divisibles.push(monkey.test_divisible_by);
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
    let lcm = multiply_vec(&divisibles);
    
    let mut inspects = Vec::new();
    for _monkey in &monkeys {
        inspects.push(Vec::new());
    }
    
    for _round in 0..10000 {
        // println!("{}", _round);
        for i in 0..monkeys.len() { 
            // let i = round % monkeys.len();
            
            let items = monkeys[i].items.clone();
            for item in items {
                // let value = optimize_value(&divisibles, operation_result(&monkeys[i], item), lcm);
                let value = operation_result(&monkeys[i], item) % lcm;
                let next_monkey_i = next_monkey_i(&monkeys[i], value);
                monkeys[i].inspect_cnt += 1;
                monkeys[next_monkey_i].items.push(value);
                // println!("Item with worry level {} is thrown to monkey {}.", value, next_monkey_i);
            }
            monkeys[i].items = Vec::new();
        }
        
        for monkey in &monkeys {
            inspects[monkey.number].push(monkey.inspect_cnt);
            // println!("{} - {}", monkey.number, monkey.inspect_cnt);
        }
    }
    
    for inspect in &inspects {
        let mut v = Vec::new();
        for i in 1..inspect.len() {
            v.push(inspect[i] - inspect[i - 1]);
        }
        // println!("{:?}", v);
    }
    
    for monkey in &monkeys {
        inspects[monkey.number].push(monkey.inspect_cnt);
        // println!("{} - {}", monkey.number, monkey.inspect_cnt);
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

fn multiply_vec(vec: &Vec<u128>) -> u128 {
    let mut n = vec[0];
    for i in 1..vec.len() {
        n *= vec[i];
    }
    return n;
}

fn next_monkey_i(monkey: &Monkey, result: u128) -> usize {
    // println!("{} / {} == {}", result, monkey.test_divisible_by, result % monkey.test_divisible_by);
    if result % monkey.test_divisible_by == 0 {
        return monkey.throw_true;
    } else {
        return monkey.throw_false;
    }
}

fn operation_result(monkey: &Monkey, item: u128) -> u128 {
    let operation_value = operation_value(monkey, item);
    // println!("{}{}{}", item, monkey.operation, operation_value);
    if monkey.operation == "+" {
        return item + operation_value;
    } else { //  monkeys[i].operation == "*"
        return item * operation_value;
    }
}

fn operation_value(monkey: &Monkey, item: u128) -> u128 {
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
