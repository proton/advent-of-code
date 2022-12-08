use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn main() {
  if let Ok(lines) = read_lines("input.txt") {
    let mut forest = Vec::new();

    for line in lines {
      if let Ok(line) = line {
        let trees: Vec<_> = line.chars().map(|x| x.to_digit(10).unwrap()).collect();
        forest.push(trees);
      }
    }

    // println!("{:?}", forest);

    let height = forest.len();
    let width  = forest[0].len();

    let mut visible_count = 0;
    for y in 0..height {
    for x in 0..width  {
      if is_visible(&forest, x, y) {
        visible_count += 1;
      }
    }}
    println!("{}", visible_count);
  }
}

fn is_visible(forest: &Vec<Vec<u32>>, x: usize, y: usize) -> bool {
  let height = forest.len();
  let width  = forest[0].len();

  let tree = forest[y][x];

  if !(0..x).any(            |nx| forest[y][nx] >= tree ) { return true; }
  if !(0..y).any(            |ny| forest[ny][x] >= tree ) { return true; }
  if !((x + 1)..width).any(  |nx| forest[y][nx] >= tree ) { return true; }
  if !((y + 1)..height).any( |ny| forest[ny][x] >= tree ) { return true; }

  return false;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
  let file = File::open(filename)?;
  Ok(io::BufReader::new(file).lines())
}
