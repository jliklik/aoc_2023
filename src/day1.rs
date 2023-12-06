use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;
use std::collections::BinaryHeap;

pub struct Day1 {
  pub part1: i64,
  pub part2: i64
}

impl Day1 {

  pub fn new<P>(path_to_input: P) -> Self 
  where P: AsRef<Path>, {
    Self {
      part1: Self::part1(&path_to_input),
      part2: Self::part2(&path_to_input)
    }
  }

  fn part1<P>(path_to_input: P) -> i64
  where P: AsRef<Path>, {
      
      let mut answer: i64 = 0;
      if let Ok(lines) = Self::read_lines(path_to_input) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
          if let Ok(ip) = line {
            let first = Self::find_first(ip.clone());
            let mut char_vector: Vec<char> = ip.chars().collect();
            char_vector.reverse();
            let reverse_ip: String = char_vector.into_iter().collect();
            let last = Self::find_first(reverse_ip.clone());
            let number = format!("{first}{last}");
            // println!("{number}"); 
            let number = number.parse::<i64>().unwrap();
            answer = answer + number;
          }
        }
      }

      answer
  }

  fn part2<P>(path_to_input: P) -> i64
  where P: AsRef<Path>, {

    let numbers = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];

    let mut answer: i64 = 0;
    if let Ok(lines) = Self::read_lines(path_to_input) {
      // Consumes the iterator, returns an (Optional) String
      for line in lines {
        if let Ok(ip) = line {
          let mut last_num_heap = BinaryHeap::<(i32, i32)>::new();
          let mut first_num_heap= BinaryHeap::<(i32, i32)>::new();
          for number in numbers {
            let formatted = format!(r"({})", number);
            let re = Regex::new(formatted.as_str()).unwrap();
            for m in re.find_iter(ip.clone().as_str()) {
              let val = match m.as_str() {
                "zero" => "0",
                "one" => "1",
                "two" => "2",
                "three" => "3",
                "four" => "4",
                "five" => "5",
                "six" => "6",
                "seven" => "7",
                "eight" => "8",
                "nine" => "9",
                _any => _any 
              };
              first_num_heap.push((m.start() as i32 * -1, val.parse::<i32>().unwrap()));
              last_num_heap.push((m.start() as i32, val.parse::<i32>().unwrap()));
            }
          }
          let (_pos, first) = first_num_heap.peek().unwrap();
          let (_pos, last) = last_num_heap.peek().unwrap();
          let number = format!("{first}{last}");
          // println!("{number}"); 
          let number = number.parse::<i64>().unwrap();
          answer = answer + number;
        }
      }
    }

    answer
  }

  fn find_first(ip: String) -> String {
    let re = Regex::new(r"(\d)").unwrap();
    let Some(caps) = re.captures(ip.as_str()) else {
      panic!("Line does not contain any numbers!");
    };
    return caps[0].to_string();
  }

  // The output is wrapped in a Result to allow matching on errors
  // Returns an Iterator to the Reader of the lines of the file.
  fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
  where P: AsRef<Path>, {
      let file = File::open(filename)?;
      Ok(io::BufReader::new(file).lines())
  }

}

