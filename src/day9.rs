use crate::aoc::{Aoc, AocRes};
use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub struct Day9 {
  path_to_input: String
}

impl Aoc for Day9 {

  fn new(path_to_input: &String) -> Self {
      Self {
          path_to_input: path_to_input.clone()
      }
  }

  fn part1(&self) -> AocRes {
    
    let mut ans: i32 = 0;
    
    if let Ok(lines) = Self::read_lines(&self.path_to_input) {
      // Consumes the iterator, returns an (Optional) String
      for line in lines {
          if let Ok(l) = line {
            let v = l.split(" ").collect::<Vec<&str>>();
            let mut nums = VecDeque::<i32>::new();
            for el in v {
              nums.push_back(el.parse::<i32>().unwrap());
            }
            // recursion problem
            //dbg!(nums.clone());
            let extrapolated = Self::differentiate(nums);
            println!("{}", extrapolated);
            // dbg!((ans, extrapolated));
            ans = ans + extrapolated;
          }
      }
    }

    AocRes::Int32(ans)
  }


  fn part2(&self) -> AocRes {
    AocRes::Int32(0)
  }

}

impl Day9 {

  fn differentiate(v: VecDeque::<i32>) -> i32 {
    let mut new_v = VecDeque::<i32>::new();
    for i in 0..v.len()-1 {
      new_v.push_back(v[i + 1] - v[i]);
    }
    if (&new_v).iter().all(|&x| x == 0) {
      dbg!((v.clone(), &new_v.clone(), 0));
      return v[v.len()-1];
    } else {
      let next_num = v[v.len()-1] + Self::differentiate(new_v.clone());
      dbg!((&v.clone(), next_num));
      return next_num;
    }
  }

  fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
  where
      P: AsRef<Path>,
  {
      let file = File::open(filename)?;
      Ok(io::BufReader::new(file).lines())
  }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works_on_sample_input() {
        let day9 = Day9::new(&"./inputs/day9_test.input".to_string());
        let AocRes::Int32(res) = day9.part1() else {
            panic!("Failed to get result from part 1!")
        };
        assert!(res == 68 + 18 + 28);
    }

    #[test]
    fn part1_works_on_sample_input_2() {
        let day9 = Day9::new(&"./inputs/day9_test2.input".to_string());
        let AocRes::Int32(res) = day9.part1() else {
            panic!("Failed to get result from part 1!")
        };
        assert!(res == 590376);
    }
}