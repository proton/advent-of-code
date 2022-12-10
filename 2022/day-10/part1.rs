use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Command {
  kind: String,
  life: i32,
  var:  i32,
}

impl Command {
  fn parse(line: String) -> Command {
    let s: Vec<_> = line.split(" ").collect();
    let kind = s[0].to_string();

    if kind == "noop" {
      return Command { kind: kind, life: 1, var: 0 };
    }
    return Command { kind: kind, life: 2, var: s[1].parse().unwrap() };
  }
}

pub fn main() {
  if let Ok(lines) = read_lines("input.txt") {
    let mut commands = Vec::new();

    for line in lines {
      if let Ok(line) = line {
        commands.push(Command::parse(line));
      }
    }

    let mut value = 1;
    let mut strengths = Vec::new();
    let mut cycle = 0;
    let mut it: i32 = -1;
    let mut life = 0;
    let mut command = &commands[0];

    loop {
      cycle += 1;

      let strength = cycle * value;
      if (cycle - 20) % 40 == 0 {
        strengths.push(strength);
      }

      if life == 0 {
        it += 1;
        if it == commands.len() as i32 {
          break;
        }
        command = &commands[it as usize];
        life = command.life;
      } else if life == 1 && command.kind == "addx" {
        value += command.var;
      }

      // println!("{} -> {} / {}: {}", cycle, value, command.kind, life);

      life -= 1;
    }

    // println!("{:?}", strengths);
    // println!("{} {}", strengths.len(), value);
    let total_strength: i32 = strengths.iter().sum();
    println!("{}", total_strength);
  }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
  let file = File::open(filename)?;
  Ok(io::BufReader::new(file).lines())
}
