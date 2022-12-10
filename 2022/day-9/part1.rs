use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;

struct Coords {
  x: i32,
  y: i32,
}

pub fn main() {
  if let Ok(lines) = read_lines("input.txt") {
    let mut visited = HashSet::new();

    let mut head = Coords { x: 0, y: 0 };
    let mut tail = Coords { x: 0, y: 0 };

    for line in lines {
      if let Ok(line) = line {
        let s: Vec<_> = line.split(" ").collect();
        let dir = get_direction(s[0]);
        let cnt: i32 = s[1].parse().unwrap();

        for _i in 0..cnt {
          head.x += dir.x;
          head.y += dir.y;

          adjust_tail(&mut tail, &head, &dir);

          // println!("{}: {} {} | {} {}", s[0], head.x, head.y, tail.x, tail.y);

          visited.insert((tail.x, tail.y));
        }
      }
    }

    // println!("{:?}", visited);
    println!("{}", visited.len());
  }
}

fn adjust_tail(tail: &mut Coords, head: &Coords, dir: &Coords) {
  if (head.x - tail.x).abs() <= 1 && (head.y - tail.y).abs() <= 1 {
    return;
  }
  if head.x == tail.x {
    tail.y += dir.y;
    return;
  }
  if head.y == tail.y {
    tail.x += dir.x;
    return;
  }
  let dx = (head.x - tail.x)/(head.x - tail.x).abs();
  let dy = (head.y - tail.y)/(head.y - tail.y).abs();
  tail.x += dx;
  tail.y += dy;
}

fn get_direction(str: &str) -> Coords {
  match str {
    "R" => return Coords { x:  1, y:  0 },
    "L" => return Coords { x: -1, y:  0 },
    "U" => return Coords { x:  0, y:  1 },
    "D" => return Coords { x:  0, y: -1 },
    &_  => return Coords { x:  0, y:  0 },
  }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
  let file = File::open(filename)?;
  Ok(io::BufReader::new(file).lines())
}
