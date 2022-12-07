use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

// $ cd /
// $ ls
// dir a
// 14848514 b.txt
// 8504156 c.dat
// dir d
// $ cd a
// $ ls
// dir e
// 29116 f
// 2557 g
// 62596 h.lst
// $ cd e
// $ ls
// 584 i
// $ cd ..
// $ cd ..
// $ cd d
// $ ls
// 4060174 j
// 8033020 d.log
// 5626152 d.ext
// 7214296 k

pub fn main() {
  if let Ok(lines) = read_lines("input.txt") {
    let mut dir_sizes:  HashMap<String, usize> = HashMap::new();
    let mut file_sizes: HashMap<String, usize> = HashMap::new();
    let mut current_path = "/".to_string();

    for line in lines {
      if let Ok(line) = line {
        if line.starts_with("$ cd") {
          let dir_name = line[5..line.len()].to_string();
          if dir_name.starts_with("/") {
            current_path = dir_name.to_string();
          } else if dir_name == ".." {
            let s: Vec<_> = current_path.split("/").collect();
            current_path = s[0..s.len() - 1].join("/");
          } else {
            current_path = format!("{}/{}", current_path, dir_name);
          }
        } else if line.starts_with("$ ls") {
          // do nothing
        } else if line.starts_with("dir ") {
          // do nothing
        } else {
          let s: Vec<_> = line.split(" ").collect();
          let size = s[0].parse().unwrap();
          let file_path = format!("{}/{}", current_path, s[1]);
          file_sizes.insert(file_path, size);
        }
        // println!("{}", line);
        // println!("{}", current_path);
      }
    }
    // println!("{:?}", file_sizes);

    for (file_path, file_size) in file_sizes.iter() {
      // println!("{:?} = {}", file_path, file_size);
      let s: Vec<_> = file_path.split("/").collect();
      let dirs = &s[1..s.len() - 1];
      let mut root = "/".to_string();
      for dir in dirs {
        let new_root = format!("{}/{}", root, dir);
        // println!("{:?} - {:?}", dirs, root);
        dir_sizes.entry(new_root.clone()).and_modify( |size| *size += *file_size ).or_insert(*file_size);
        root = new_root;
      }
    }
    // println!("{:?}", dir_sizes);
    let total: usize = dir_sizes.values().filter( |size| **size <= 100000 ).sum();
    println!("{}", total);
  }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
  let file = File::open(filename)?;
  Ok(io::BufReader::new(file).lines())
}
