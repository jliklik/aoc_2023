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

      dbg!(&start);
      dbg!(&possible_dirs);

      if possible_dirs.len() > 2 {
        panic!("Assume only 2 directions that S connects to...")
      } else {
        let worker_result_1 = Arc::new(Mutex::new((0, 0, 0)));
        let worker_result_2 = Arc::new(Mutex::new((0, 0, 0)));
        let worker_result_1_copy = worker_result_1.clone();
        let worker_result_2_copy = worker_result_2.clone();

        let matrix_ref_1 = Arc::new(matrix);
        let matrix_ref_2 = matrix_ref_1.clone();

        let adjacent_to_start_pos_1 = (possible_dirs[0].1, possible_dirs[0].2);
        let child_1 = thread::spawn(move || 
          {
            Self::follow_loop(Arc::clone(&worker_result_1), adjacent_to_start_pos_1, adjacent_to_start_pos_1, start, matrix_ref_1, 1);
          }
        );
        // let adjacent_to_start_pos_2 = (possible_dirs[1].1, possible_dirs[1].2);
        // let child_2 = thread::spawn(move || 
        //   {
        //     Self::follow_loop(Arc::clone(&worker_result_2), adjacent_to_start_pos_2, adjacent_to_start_pos_2, start, matrix_ref_2, 1);
        //   }
        // );

        while true {
          let mut result_1 = *worker_result_1_copy.lock().unwrap();
          let ten_millis = std::time::Duration::from_millis(10);

          while result_1.2 == 0 {
            drop(result_1);
            std::thread::sleep(ten_millis);
            result_1 = *worker_result_1_copy.lock().unwrap();
          }

          let mut result_2 = *worker_result_2_copy.lock().unwrap();
          while result_2.2 == 0 {
            drop(result_2);
            std::thread::sleep(ten_millis);
            result_2 = *worker_result_2_copy.lock().unwrap();
          }

          dbg!((result_1, result_2));

          // If same location and same step count, then we have found the midpoint
          if result_1 == result_2 {
            ans = result_1.2;
            break;
          }

        }

        let _ = child_1.join();
        // let _ = child_2.join();
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
    for y in 0..matrix.len() {
      for x in 0..matrix[y].len() {
        if matrix[y][x] == 'S' {
          start = (x, y)
        }        
      }
    }
    start
  }

  fn find_possible_directions_around_start(start: (usize, usize), matrix: &Vec<Vec<char>>) -> Vec::<(char, usize, usize)> {

    let mut possible_starts = Vec::<(char, usize, usize)>::new();

    let (x, y) = Self::go_north(start);
    if Self::valid((x, y), matrix) && vec!['|', '7', 'F'].contains(&matrix[y][x]) {
      possible_starts.push((matrix[y][x], x, y))
    }

    let (x, y) = Self::go_south(start);
    if Self::valid((x, y), matrix) && vec!['|', 'L', 'J'].contains(&matrix[y][x]) {
      possible_starts.push((matrix[y][x], x, y))
    }

    let (x, y) = Self::go_east(start);
    if Self::valid((x, y), matrix) && vec!['-', 'J', '7'].contains(&matrix[y][x]) {
      possible_starts.push((matrix[y][x], x, y))
    }

    let (x, y) = Self::go_west(start);
    if Self::valid((x, y), matrix) && vec!['-', 'L', 'F'].contains(&matrix[y][x]) {
      possible_starts.push((matrix[y][x], x, y))
    }

    possible_starts
  }

  fn follow_loop(worker_result: Arc<Mutex<(usize, usize, i32)>>, start_pos: (usize, usize), current_pos: (usize, usize), prev_pos: (usize, usize), matrix: Arc<Vec<Vec<char>>>, count: i32) {
    
    match Self::do_follow_loop(start_pos, current_pos, prev_pos, matrix.clone()) {
      Some((next_x, next_y)) => {
        
        // let boss know result
        let mut result = *worker_result.lock().unwrap();
        let ten_millis = std::time::Duration::from_millis(10);
        while result.2 != 0 {
          drop(result);
          std::thread::sleep(ten_millis);
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

  fn do_follow_loop(start_pos: (usize, usize), current_pos: (usize, usize), prev_pos: (usize, usize), matrix: Arc<Vec<Vec<char>>>) -> Option<(usize, usize)> {
    let next_pos = Self::next_position(matrix[current_pos.1][current_pos.0], current_pos, prev_pos);
    if next_pos == start_pos {
      None
    } else {
      Some(next_pos)
    }
  }

  // where is a relative to b?
  fn position(a: (usize, usize), b: (usize, usize)) -> Position {
    if a.0 + 1 == b.0 && a.1 == b.1 {
      Position::West
    } else if a.0 == b.0 + 1 && a.1 == b.1 {
      Position::East
    } else if a.0 == b.0 && a.1 + 1 == b.1 {
      Position::North
    } else if a.0 == b.0 && a.1 == b.1 + 1 {
      Position::South
    } else if a.0 + 1 == b.0 && a.1 + 1 == b.1 {
      Position::NorthWest
    } else if a.0 == b.0 + 1 && a.1 + 1 == b.1 {
      Position::NorthEast
    } else if a.0 + 1 == b.0 && a.1 == b.1 + 1 {
      Position::SouthWest
    } else {
      Position::SouthEast
    }
  }

  fn go_east(a: (usize, usize)) -> (usize, usize) {(a.0 + 1, a.1)}

  fn go_west(a: (usize, usize)) -> (usize, usize) {
    if a.0 >= 1 {
      (a.0 - 1, a.1)
    } else {
      (a.0, a.1)
    }
  }

  fn go_north(a: (usize, usize)) -> (usize, usize) {
    if a.1 >= 1 {
      (a.0, a.1 - 1)
    } else {
      (a.0, a.1)
    }    
  }

  fn go_south(a: (usize, usize)) -> (usize, usize) {(a.0, a.1 + 1)}

  fn valid(a: (usize, usize), matrix: &Vec<Vec<char>>) -> bool {
    if a.1 >= matrix.len() || a.0 >= matrix[0].len() {
      false
    } else {
      true
    }
  }

  fn next_position(symbol: char, symbol_position: (usize, usize), prev_position: (usize, usize)) -> (usize, usize) 
  {

    dbg!(symbol);

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