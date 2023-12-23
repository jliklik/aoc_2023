use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

// Dynamic dispatch just for learning
pub trait Neighbour {
  fn get_left(&self) -> String;
  fn get_right(&self) -> String;
}

pub struct DesertNode {
  left: String,
  right: String
}

impl Neighbour for DesertNode {
  fn get_left(&self) -> String {
    return self.left.clone();
  }
  fn get_right(&self) -> String {
    return self.right.clone();
  }
}

pub struct DesertMap {
  pub nodes: HashMap<String, Box<dyn Neighbour>>
}

impl DesertMap {
  pub fn new() -> Self {
    Self {
      nodes: HashMap::new()
    }
  }

  pub fn insert_node(dmap: &mut Self, key: String, node: Box<dyn Neighbour>) {
    dmap.nodes.insert(key, node);
  }

  pub fn list_nodes(dmap: &Self) {
    for (k, v) in &dmap.nodes {
      println!("self: {}, left: {}, right: {}", k, v.get_left(), v.get_right())
    }
  }

  pub fn find_path(dmap: &Self, directions: String) -> i32 {
    let dir_vec: Vec<char> = directions.chars().collect();

    let mut key = "AAA".to_string();
    let mut steps = 0;
    while key != "ZZZ" {
      for dir in &dir_vec {
          (steps, key) = Self::recur_find_path(dmap, dir, key, steps);
          if key == "ZZZ" {
            dbg!("Found zzz");
            dbg!((&key, &dir, &steps));
            break;
          }
          dbg!((&key, &dir, &steps));
      }
    }
    steps
  }

  pub fn recur_find_path(dmap: &Self, direction: &char, key: String, steps: i32) -> 
    (i32, String)
  {
    let Some(node) = dmap.nodes.get(&key) else {
      panic!("Node not found in map!");
    };
    let next_key = match direction {
      'L' => node.get_left(),
      'R' => node.get_right(),
      _ => panic!("unknown!")
    };
    (steps + 1, next_key)
  }
}

pub struct Day8 {
  pub part1: i32,
  pub part2: i32
}

impl Day8 {

  pub fn new<P>(path_to_input: P) -> Self 
  where P: AsRef<Path>, {
    Self {
      part1: Self::part1(&path_to_input),
      part2: Self::part2(&path_to_input)
    }
  }

  fn part1<P>(path_to_input: P) -> i32
  where P: AsRef<Path>, {

    let file = File::open(path_to_input).unwrap();
    let mut buffer = io::BufReader::new(file);

    let mut dmap = DesertMap::new();
    let Some(mut directions) = Self::read_line(&mut buffer) else {
      panic!("Failed to read directions");
    };
    // Remove last "\n" character
    directions = directions[..(directions.len() - 1)].to_string();
    while let Some(contents) = Self::read_line(&mut buffer) {
      if contents != "\n" {
        let s = contents.split(" = ").collect::<Vec<&str>>();
        let key = s[0].to_string();
        let n = s[1].to_string();
        let neighbours = n.split(", ").collect::<Vec<&str>>();
        let n1 = neighbours[0][1..].to_string();
        let n2 = neighbours[1][..3].to_string();
        let node = Box::new(
          DesertNode {
            left: n1,
            right: n2
          }
        );
        DesertMap::insert_node(&mut dmap, key, node);
      }
    }
    DesertMap::list_nodes(&dmap);
    let ans = DesertMap::find_path(&dmap, directions);

    ans
  }

  fn part2<P>(path_to_input: P) -> i32
  where P: AsRef<Path>, {
    0
  }

  fn read_line(buffer: &mut io::BufReader<File>) -> Option<String> {
    let mut line = String::new();
    if let Ok(num_bytes) = buffer.read_line(&mut line) {
      match num_bytes {
        0 => None,
        _ => Some(line)
      }
    } else {
      panic!("Error occured while reading file!");
    }
  }

  // fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
  // where P: AsRef<Path>, {
  //   let file = File::open(filename)?;
  //   Ok(io::BufReader::new(file).lines())
  // }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works_on_sample_input() {
      let part1 = Day8::part1("./inputs/day8_test.input");
      dbg!(part1);
      assert!(part1 == 6);
    }

    #[test]
    fn works_on_sample_input_2() {
      let part1 = Day8::part1("./inputs/day8_test2.input");
      dbg!(part1);
      assert!(part1 == 2);
    }
}