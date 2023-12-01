use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;

pub struct Day1 {
  pub answer: i64
}

impl Day1 {

  pub fn new<P>(path_to_input: P) -> Self 
  where P: AsRef<Path>, {
    Self {
      answer: Self::process(path_to_input)
    }
  }

  fn process<P>(path_to_input: P) -> i64
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

