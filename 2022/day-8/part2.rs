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

    let mut max_score = 0;

    for y in 0..height {
    for x in 0..width  {
      let s = score(&forest, x, y);
      if s > max_score {
        max_score = s;
      }
    }}
    println!("{}", max_score);
  }
}

fn score(forest: &Vec<Vec<u32>>, x: usize, y: usize) -> u32 {
  if !is_visible(forest, x, y) {
    return 0;
  }

  let scores = [
    side_score(forest, x, y,  1,  0),
    side_score(forest, x, y, -1,  0),
    side_score(forest, x, y,  0,  1),
    side_score(forest, x, y,  0, -1),
  ];
  let score = scores[0] * scores[1] * scores[2] * scores[3];
  // println!("{} {} - {} - {:?} - {}", x, y, forest[y][x], scores, score);
  return score;
}

fn side_score(forest: &Vec<Vec<u32>>, x: usize, y: usize, dx: i32, dy: i32) -> u32 {
  let height = forest.len() as i32;
  let width  = forest[0].len() as i32;

  let mut nx = x as i32;
  let mut ny = y as i32;

  if nx < 0 || ny < 0 || nx == width || ny == height {
    return 0;
  }

  let home = forest[y][x];
  let mut score = 0;

  loop {
    nx += dx;
    ny += dy;
    if nx < 0 || ny < 0 || nx == width || ny == height {
      break;
    }
    score += 1;

    let current = forest[ny as usize][nx as usize];
    if current >= home {
      break;
    }
  }

  return score;
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
