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

    let mut knots = Vec::new();
    for _i in 0..10 {
      let knot = Coords { x: 0, y: 0 };
      knots.push(knot);
    }

    for line in lines {
      if let Ok(line) = line {
        let s: Vec<_> = line.split(" ").collect();
        let dir = get_direction(s[0]);
        let cnt: i32 = s[1].parse().unwrap();

        for _i in 0..cnt {
          knots[0].x += dir.x;
          knots[0].y += dir.y;

          for pos in 0..9 {
            adjust_knot(&mut knots, pos);
          }

          // println!("{}: {} {} | {} {}", s[0], knots[0].x, knots[0].y, knots[9].x, knots[9].y);

          visited.insert((knots[9].x, knots[9].y));
        }
      }
    }

    // println!("{:?}", visited);
    println!("{}", visited.len());
  }
}

fn dvec(diff: i32) -> i32 {
  if diff == 0 {
    return 0;
  }
  return diff / diff.abs();
}

fn adjust_knot(knots: &mut Vec<Coords>, pos: usize) {
  let dx = knots[pos].x - knots[pos + 1].x;
  let dy = knots[pos].y - knots[pos + 1].y;

  // println!(">>> {} = {} {}", pos, dx, dy);

  if dx.abs() <= 1 && dy.abs() <= 1 {
    return;
  }
  knots[pos + 1].x += dvec(dx);
  knots[pos + 1].y += dvec(dy);
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
