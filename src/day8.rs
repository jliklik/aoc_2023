use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// Dynamic dispatch just for learning
pub trait Neighbour {
    fn get_left(&self) -> String;
    fn get_right(&self) -> String;
}

pub struct DesertNode {
    left: String,
    right: String,
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
    pub nodes: HashMap<String, Box<dyn Neighbour>>,
}

impl DesertMap {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
        }
    }

    pub fn insert_node(dmap: &mut Self, key: String, node: Box<dyn Neighbour>) {
        dmap.nodes.insert(key, node);
    }

    pub fn list_nodes(dmap: &Self) {
        for (k, v) in &dmap.nodes {
            println!(
                "self: {}, left: {}, right: {}",
                k,
                v.get_left(),
                v.get_right()
            )
        }
    }

    fn find_path(
        dmap: &Self,
        start_key: &String,
        directions: &String,
        end_cond_fn: &dyn Fn(&String) -> bool,
    ) -> i32 {
        let dir_vec: Vec<char> = directions.chars().collect();
        let mut key = start_key.clone();
        let mut steps = 0;
        while !(end_cond_fn(&key)) {
            for dir in &dir_vec {
                key = Self::find_next_key(dmap, dir, &key);
                steps = steps + 1;
                if end_cond_fn(&key) {
                    // dbg!((&key, &dir, &steps));
                    break;
                }
            }
        }
        steps
    }

    // taken from https://gist.github.com/victor-iyi/8a84185c1d52419b0d4915a648d5e3e1
    pub fn gcd(mut n: u64, mut m: u64) -> u64 {
        assert!(n != 0 && m != 0);
        while m != 0 {
            if m < n {
                std::mem::swap(&mut m, &mut n);
            }
            m %= n;
        }
        n
    }

    // Brute forcing this is NOT the way to go
    // Try using Lowest Common Denominator to solve this problem instead
    // Algo:
    // - Find num steps for each one
    // - Find lowest common denominator for all steps
    pub fn find_path_2(dmap: &Self, directions: &String) -> u64 {
        let start_keys = Self::get_starting_keys(dmap);
        fn ends_with_z(s: &String) -> bool {
            let last = s.chars().last().unwrap();
            last == 'Z'
        }
        let mut steps_vector = Vec::new();
        for start_key in &start_keys {
            let steps = Self::find_path(dmap, start_key, directions, &ends_with_z);
            dbg!((start_key, steps));
            steps_vector.push(steps as u64);
        }

        let Some(mut a) = steps_vector.pop() else {
            panic!("empty steps");
        };
        let mut gcd = a;
        let mut lcm: u128 = a as u128;
        for _i in 0..steps_vector.len() {
            let Some(b) = steps_vector.pop() else {
                panic!("empty steps");
            };
            gcd = Self::gcd(a, b);
            lcm = (lcm as u128) * (b as u128) / gcd as u128;
            a = gcd;
            println!("gcd: {}", gcd);
            println!("lcm: {}", lcm);
        }
        lcm as u64
    }

    pub fn find_next_key(dmap: &Self, direction: &char, key: &String) -> String {
        let Some(node) = dmap.nodes.get(key) else {
            panic!("Node not found in map!");
        };
        let next_key = match direction {
            'L' => node.get_left(),
            'R' => node.get_right(),
            _ => panic!("unknown!"),
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
    pub part2: u64,
}

impl Day8 {
    pub fn new<P>(path_to_input: P) -> Self
    where
        P: AsRef<Path>,
    {
        Self {
            part1: Self::part1(&path_to_input),
            part2: Self::part2(&path_to_input),
        }
    }

    fn create_dmap<P>(path_to_input: P) -> (DesertMap, String)
    where
        P: AsRef<Path>,
    {
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
                let node = Box::new(DesertNode {
                    left: n1,
                    right: n2,
                });
                DesertMap::insert_node(&mut dmap, key, node);
            }
        }
        (dmap, directions)
    }

    fn part1<P>(path_to_input: P) -> i32
    where
        P: AsRef<Path>,
    {
        let (dmap, directions) = Self::create_dmap(path_to_input);
        // DesertMap::list_nodes(&dmap);
        fn zzz(s: &String) -> bool {
            s.clone() == "ZZZ".to_string()
        }
        let ans = DesertMap::find_path(&dmap, &"AAA".to_string(), &directions, &zzz);

        ans
    }

    fn part2<P>(path_to_input: P) -> u64
    where
        P: AsRef<Path>,
    {
        let (dmap, directions) = Self::create_dmap(path_to_input);
        // DesertMap::list_nodes(&dmap);
        let ans = DesertMap::find_path_2(&dmap, &directions);
        ans
    }

    fn read_line(buffer: &mut io::BufReader<File>) -> Option<String> {
        let mut line = String::new();
        if let Ok(num_bytes) = buffer.read_line(&mut line) {
            match num_bytes {
                0 => None,
                _ => Some(line),
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
