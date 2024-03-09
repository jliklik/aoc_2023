use crate::aoc::{Aoc, AocRes};
use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread;


pub struct Day10 {
  path_to_input: String
}

impl Aoc for Day10 {

  fn new(path_to_input: &String) -> Self {
    Self {
      path_to_input: path_to_input.clone()
    }
  }

  /// Algorithm
  /// Read text file into a matrix
  /// Find S
  /// Look around
  /// Spin up two threads
  /// At each step, threads report their current position
  /// We know we have found the furthest distance when both threads report the same position
  /// Each thread needs to know:
  /// - current position
  /// - previous positon
  /// - based off symbol @ current position and previous position, can figure out where to go next
  fn part1(&self) -> AocRes {

    let mut ans: i32 = 0;

    if let Ok(lines) = Self::read_lines(&self.path_to_input) {
      // Consumes the iterator, returns an (Optional) String
      let mut matrix = Vec::<Vec<char>>::new();
      for line in lines {
        if let Ok(l) = line {
          let row: Vec<char> = l.chars().collect();
          matrix.push(row);
        }
      }

      let start = Self::find_start(&matrix);
      let possible_dirs = Self::find_possible_directions_around_start(start, &matrix);

      if possible_dirs.len() > 2 {
        panic!("Assume only 2 directions that S connects to...")
      } else {

        let mut worker_result = VecDeque::<Arc::<Mutex<(usize, usize, i32)>>>::new();
        worker_result.push_back(Arc::new(Mutex::new((0, 0, 0))));
        worker_result.push_back(Arc::new(Mutex::new((0, 0, 0))));
  
        let mut children = Vec::new();

        for id in 0..2 {
          let adjacent_to_start_pos = (possible_dirs[id].1, possible_dirs[id].2);
          let child = thread::spawn(move || 
            {
              Self::follow_loop(Arc::clone(&worker_result[id]), adjacent_to_start_pos, adjacent_to_start_pos, start, &matrix, 1);
            }
          );
          children.push(child);
        }
      }

    
    }

    AocRes::Int32(ans)

  }

  fn part2(&self) -> AocRes {
    AocRes::Int32(0)
  }

}

enum Position {
  North,
  South,
  East,
  West,
  NorthWest,
  NorthEast,
  SouthEast,
  SouthWest
}

impl Day10 {
  fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
  where
      P: AsRef<Path>,
  {
      let file = File::open(filename)?;
      Ok(io::BufReader::new(file).lines())
  }

  fn find_start(matrix: &Vec<Vec<char>>) -> (usize, usize) {
    let mut start: (usize, usize) = (0, 0);
    for r in 0..matrix.len() {
      for c in 0..matrix[r].len() {
        if matrix[r][c] == 'S' {
          start = (r, c)
        }        
      }
    }
    start
  }

  fn find_possible_directions_around_start(start: (usize, usize), matrix: &Vec<Vec<char>>) -> Vec::<(char, usize, usize)> {

    let mut possible_starts = Vec::<(char, usize, usize)>::new();

    let (x, y) = Self::go_north(start);
    if Self::valid((x, y), matrix) && vec!['|', '7', 'F'].contains(&matrix[x][y]) {
      possible_starts.push((matrix[x][y], x, y))
    }

    let (x, y) = Self::go_south(start);
    if Self::valid((x, y), matrix) && vec!['|', 'L', 'J'].contains(&matrix[x][y]) {
      possible_starts.push((matrix[x][y], x, y))
    }

    let (x, y) = Self::go_east(start);
    if Self::valid((x, y), matrix) && vec!['-', 'J', '7'].contains(&matrix[x][y]) {
      possible_starts.push((matrix[x][y], x, y))
    }

    let (x, y) = Self::go_west(start);
    if Self::valid((x, y), matrix) && vec!['-', 'L', 'F'].contains(&matrix[x][y]) {
      possible_starts.push((matrix[x][y], x, y))
    }

    possible_starts
  }

  fn follow_loop(worker_result: Arc<Mutex<(usize, usize, i32)>>, start_pos: (usize, usize), current_pos: (usize, usize), prev_pos: (usize, usize), matrix: &Vec<Vec<char>>, count: i32) {
    
    match Self::do_follow_loop(start_pos, current_pos, prev_pos, matrix) {
      Some((next_x, next_y)) => {
        
        // let boss know result
        let mut result = *worker_result.lock().unwrap();
        while result.2 != 0 {
          drop(result);
          result = *worker_result.lock().unwrap();
        }

        result = (next_x, next_y, count + 1);
        // allow boss to continue
        drop(result);

        Self::follow_loop(worker_result, start_pos, (next_x, next_y), current_pos, matrix, count + 1)
      }
      _ => () // finished
    }
  }

  fn do_follow_loop(start_pos: (usize, usize), current_pos: (usize, usize), prev_pos: (usize, usize), matrix: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    let next_pos = Self::next_position(matrix[current_pos.0][current_pos.1], current_pos, prev_pos);
    if next_pos == start_pos {
      None
    } else {
      Some(next_pos)
    }
  }

  fn position(a: (usize, usize), b: (usize, usize)) -> Position {
    if a.0 == b.0 - 1 && a.1 == b.1 {
      Position::West
    } else if a.0 == b.0 + 1 && a.1 == b.1 {
      Position::East
    } else if a.0 == b.0 && a.1 == b.1 - 1 {
      Position::North
    } else if a.0 == b.0 && a.1 == b.1 + 1 {
      Position::South
    } else if a.0 == b.0 - 1 && a.1 == b.1 - 1 {
      Position::NorthWest
    } else if a.0 == b.0 + 1 && a.1 == b.1 - 1 {
      Position::NorthEast
    } else if a.0 == b.0 - 1 && a.1 == b.1 + 1 {
      Position::SouthWest
    } else {
      Position::SouthEast
    }
  }

  fn go_east(a: (usize, usize)) -> (usize, usize) {(a.0 + 1, a.1)}

  fn go_west(a: (usize, usize)) -> (usize, usize) {(a.0 - 1, a.1)}

  fn go_north(a: (usize, usize)) -> (usize, usize) {(a.0, a.1 - 1)}

  fn go_south(a: (usize, usize)) -> (usize, usize) {(a.0, a.1 + 1)}

  fn valid(a: (usize, usize), matrix: &Vec<Vec<char>>) -> bool {
    if a.0 < 0 || a.1 < 0 || a.1 >= matrix.len() || a.0 >= matrix[0].len() {
      false
    } else {
      true
    }
  }

  fn next_position(symbol: char, symbol_position: (usize, usize), prev_position: (usize, usize)) -> (usize, usize) 
  {
    match symbol {
      '-' => {
        match Self::position(prev_position, symbol_position) {
          Position::West => Self::go_east(symbol_position),
          Position::East => Self::go_west(symbol_position),
          _ => panic!("- pipe but positions do not make sense!")
        }
      }
      '|' => {
        match Self::position(prev_position, symbol_position) {
          Position::North => Self::go_south(symbol_position),
          Position::South => Self::go_north(symbol_position),
          _ => panic!("- pipe but positions do not make sense!")
        }
      }
      'L' => {
        match Self::position(prev_position, symbol_position) {
          Position::North => Self::go_east(symbol_position),
          Position::East => Self::go_north(symbol_position),
          _ => panic!("- pipe but positions do not make sense!")
        }
      }
      'J' => {
        match Self::position(prev_position, symbol_position) {
          Position::North => Self::go_west(symbol_position),
          Position::West => Self::go_north(symbol_position),
          _ => panic!("- pipe but positions do not make sense!")
        }
      }
      '7' => {
        match Self::position(prev_position, symbol_position) {
          Position::South => Self::go_west(symbol_position),
          Position::West => Self::go_south(symbol_position),
          _ => panic!("- pipe but positions do not make sense!")
        }
      }
      'F' => {
        match Self::position(prev_position, symbol_position) {
          Position::South => Self::go_east(symbol_position),
          Position::East => Self::go_south(symbol_position),
          _ => panic!("- pipe but positions do not make sense!")
        }
      }
      _ => panic!("Unrecognized symbol!")
    }
  }

}