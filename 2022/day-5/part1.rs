use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Command {
  count: u32,
  from:  u32,
  to:    u32,
}

pub fn main() {
  if let Ok(lines) = read_lines("input.txt") {
    let mut columns  = Vec::new();
    let mut commands = Vec::new();
    
    for line in lines {
      if let Ok(line) = line {
        if line.contains("[") {
          let cnt = (line.len() + 1) / 4;
          if columns.len() == 0 {
            for _i in 0..cnt {
              let v = Vec::<char>::new();
              columns.push(v);
            }
          }
          // println!("{}", line);
          for i in 0..cnt {
            // println!("{} {} {}", i, 1 + 4 * i, line.len());
            let char = line.chars().nth(1 + 4 * i).unwrap();
            if char != ' ' {
              columns[i].push(char);
            }
          }
        } else if line.starts_with("move") {
          commands.push(parse_command(&line));
        }
      }
    }

    for i in 0..columns.len() {
      columns[i].reverse();
    }

    for command in commands {
      for _i in 0..command.count {
        let c = columns[(command.from - 1) as usize].pop().unwrap();
        columns[(command.to - 1) as usize].push(c);
      }
    }

    print_columns(&columns);

    println!("");
    for column in columns {
      print!("{}", column.last().unwrap());
    }
    println!("");
  }
}

fn print_columns(columns: &Vec<Vec<char>>) {
  let max_height = columns.iter().map(|c| c.len()).max().unwrap();
  for i in 0..max_height {
    for (j, column) in columns.iter().enumerate() {
      let index = max_height - 1 - i;
      if index < column.len() {
        let char = column[index];
        print!("[{}]", char);
      } else {
        print!("   ");
      }
      if j < columns.len() - 1 {
        print!(" ");
      }
    }
    println!("");
  }
  for j in 0..columns.len() {
    print!(" {} ", j + 1);
    if j < columns.len() - 1 {
      print!(" ");
    }
  }
  println!("");
}

fn parse_command(line: &String) -> Command {
  // let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
  // let caps = re.captures(line).unwrap();
  // let count = caps.get(1).unwrap().as_str().parse().unwrap();
  // let from  = caps.get(2).unwrap().as_str().parse().unwrap();
  // let to    = caps.get(3).unwrap().as_str().parse().unwrap();

  let parts: Vec<&str> = line.split(" ").collect();
  let count = parts[1].parse().unwrap();
  let from  = parts[3].parse().unwrap();
  let to    = parts[5].parse().unwrap();

  return Command {
    count: count,
    from:  from,
    to:    to,
  }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
  let file = File::open(filename)?;
  Ok(io::BufReader::new(file).lines())
}