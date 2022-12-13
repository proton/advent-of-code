use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

struct BreadthFirstSearch {
    graph: HashMap<usize, HashSet<usize>>,
    visited: HashSet<usize>,
}

impl BreadthFirstSearch {
    pub fn new() -> BreadthFirstSearch {
        return BreadthFirstSearch {
            graph: HashMap::new(),
            visited: HashSet::new(),
        };
    }
    
    pub fn connect(&mut self, a: usize, b: usize) {
        if !self.graph.contains_key(&a) {
            self.graph.insert(a, HashSet::new());
        }
        self.graph.entry(a).and_modify(|neighbours| { neighbours.insert(b); } );
    }
    
    pub fn distance(&mut self, from: usize, to: usize) -> usize {
        self.visited = HashSet::new();
        let mut search_queue = VecDeque::new();
        for neigbour in self.graph.get(&from).unwrap() {
            search_queue.push_back((1, neigbour));
        }
        
        while !search_queue.is_empty() {
            let entry = search_queue.pop_front().unwrap();
            let distance = entry.0;
            let node = *entry.1;
            // println!("{} =? {} | {} {}", node, to, distance, search_queue.len());
            if node == to {
                return distance;
            }
            for neighbour in self.graph.get(&node).unwrap() {
                if !self.visited.contains(neighbour) {
                    self.visited.insert(*neighbour);
                    search_queue.push_back((distance + 1, neighbour));
                }
            }
        }
        return usize::MAX;
    }
}

pub fn main() {
  if let Ok(lines) = read_lines("input.txt") {
    let mut map = Vec::new();

    for line in lines {
      if let Ok(line) = line {
        let row: Vec<_> = line.chars().collect();
        map.push(row);
      }
    }
    
    let height = map.len();
    let width = map[0].len();
    
    let neighbours = [[0, 1], [0, -1], [1, 0], [-1, 0]];
    
    let mut to_coord = 0;
    
    let mut search = BreadthFirstSearch::new();
    
    let mut start_coords = Vec::new();
    
    for y in 0..height {
      for x in 0..width {
        let c = map[y][x];
        if c == 'S' || c == 'a' {
            let start_coord = coords_to_id(x, y, width);
            start_coords.push(start_coord);
        } else if c == 'E' {
            to_coord = coords_to_id(x, y, width);
        }
          
        let a_id = coords_to_id(x, y, width);
        for neighbour_shift in neighbours {
            let dx = neighbour_shift[0];
            let dy = neighbour_shift[1];
              
            if x == 0 && dx == -1 { continue; }
            if y == 0 && dy == -1 { continue; }
            if x == width - 1 && dx == 1 { continue; }
            if y == height - 1 && dy == 1 { continue; }
            
            let nx = (x as i64 + dx) as usize;
            let ny = (y as i64 + dy) as usize;
            
            // println!(">>> {} {} | {} {} | {} {}", x, y, nx, ny, dx, dy);
              
            let b_id = coords_to_id(nx, ny, width);
            if walkable(&map, x, y, nx, ny) {
                search.connect(a_id, b_id);
            }
            if walkable(&map, nx, ny, x, y) {
                search.connect(b_id, a_id);
            }
        }
      }
    }
    
    let r = start_coords.iter().map(|start_coord| search.distance(*start_coord, to_coord)).min();
    println!("{}", r.unwrap());
  }
}

fn walkable(map: &Vec<Vec<char>>, x: usize, y: usize, nx: usize, ny: usize) -> bool {
    let mut a_cell = map[y][x];
    let mut b_cell = map[ny][nx];
    
    if a_cell == 'S' {
        a_cell = 'a';
    }
    
    if b_cell == 'E' {
        b_cell = 'z';
    }
    
    return a_cell as i32 + 1 >= b_cell as i32;
}

fn coords_to_id(x: usize, y: usize, width: usize) -> usize {
    return y * width + x;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
  let file = File::open(filename)?;
  Ok(io::BufReader::new(file).lines())
}
