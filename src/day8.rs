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

  pub fn find_path_2(dmap: &Self, directions: String) -> i32 {
    let dir_vec: Vec<char> = directions.chars().collect();

    let mut steps = 0;
    let mut keys = Self::get_starting_keys(dmap);
    for k in &keys {
      println!("steps: {}, new key: {}", steps, k);
    }
    let mut all_zs = false;
    while !all_zs {
      for dir in &dir_vec {
        let mut new_keys = Vec::<String>::new();
        for key in &keys {
          let new_key = Self::find_next_key(dmap, dir, key);
          new_keys.push(new_key);
        }
        steps = steps + 1;
        for k in &new_keys {
          println!("steps: {}, new key: {}", steps, k);
        }
        all_zs = new_keys.iter().all(|x| {
          let last = x.chars().last().unwrap();
          if last == 'Z' { true } else { false }
        });
        if all_zs {
          // Finished
          break;
        }
        keys = new_keys;
      }
    }

    steps
    
  }

  pub fn find_next_key(dmap: &Self, direction: &char, key: &String) -> String {
    let Some(node) = dmap.nodes.get(key) else {
      panic!("Node not found in map!");
    };
    let next_key = match direction {
      'L' => node.get_left(),
      'R' => node.get_right(),
      _ => panic!("unknown!")
    };
    next_key
  }

  fn get_starting_keys(dmap: &Self) -> Vec<String> {
    let mut starts = Vec::<String>::new();
    for (k, v) in &dmap.nodes {
      let last = k.chars().last().unwrap();
      if last == 'A' {
        starts.push(k.clone());
      }
    }
    starts
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

  fn create_dmap<P>(path_to_input: P) -> (DesertMap, String)
  where P: AsRef<Path> {
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
    (dmap, directions)
  }

  fn part1<P>(path_to_input: P) -> i32
  where P: AsRef<Path>, {

    let (dmap, directions) = Self::create_dmap(path_to_input);
    DesertMap::list_nodes(&dmap);
    let ans = DesertMap::find_path(&dmap, directions);

    ans
  }

  fn part2<P>(path_to_input: P) -> i32
  where P: AsRef<Path>, {
    let (dmap, directions) = Self::create_dmap(path_to_input);
    DesertMap::list_nodes(&dmap);
    let ans = DesertMap::find_path_2(&dmap, directions);
    ans
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

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works_on_sample_input() {
      let part1 = Day8::part1("./inputs/day8_test.input");
      dbg!(part1);
      assert!(part1 == 6);
    }

    #[test]
    fn part1_works_on_sample_input_2() {
      let part1 = Day8::part1("./inputs/day8_test2.input");
      dbg!(part1);
      assert!(part1 == 2);
    }

    #[test]
    fn part2_works_on_sample_input() {
      let part2 = Day8::part2("./inputs/day8_p2_test.input");
      dbg!(part2);
      assert!(part2 == 6);
    }
}