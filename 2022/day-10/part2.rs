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

    let sprite: Vec<_> = "###.....................................".chars().collect();
    
    let mut value: i32 = 0;
    let mut cycle = 0;
    let mut it: i32 = -1;
    let mut life = 0;
    let mut command = &commands[0];
    
    let mut screen = Vec::new();

    loop {
      let row_n = cycle / 40;
      if row_n == screen.len() {
          let row = ['.'; 40];
          screen.push(row);
      }
      
      let col_n: usize = (cycle % 40) as usize;
      let symbol = sprite[((cycle as i32 + 40 - value) % 40) as usize];
      
      // println!("{} {} {}", cycle, value, symbol);
      
      screen[row_n][col_n] = symbol;

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

      life -= 1;
      cycle += 1;
    }
    
    for row in screen {
        let s: String = row.iter().collect();
        println!("{}", s);
    }
  }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
  let file = File::open(filename)?;
  Ok(io::BufReader::new(file).lines())
}
