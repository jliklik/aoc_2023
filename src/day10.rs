use crate::aoc::{Aoc, AocRes};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::sync::{Arc, Mutex, Condvar};
use std::thread;


pub struct Day10 {
  path_to_input: String
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Coordinate {
  x: usize,
  y: usize
}

#[derive(Debug, Clone)]
struct Step {
  coord: Coordinate,
  value: i32
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

        let initial_step = Step {
          coord: Coordinate {
            x: 0, y: 0
          },
          value: 0
        };
        let pair1 = Arc::new((Mutex::new(initial_step.clone()), Condvar::new()));
        let pair2: Arc<(Mutex<Step>, Condvar)> = Arc::new((Mutex::new(initial_step.clone()), Condvar::new()));
        let pair1_copy = Arc::clone(&pair1);
        let pair2_copy = Arc::clone(&pair2);

        let matrix_ref = Arc::new(matrix);
        let matrix_ref_1 = Arc::clone(&matrix_ref);
        let matrix_ref_2 = Arc::clone(&matrix_ref);

        let adjacent_to_start_pos_1 = Coordinate{
          x: possible_dirs[0].1.x, 
          y: possible_dirs[0].1.y
        };

        let adjacent_to_start_pos_2 =  Coordinate{
          x: possible_dirs[1].1.x, 
          y: possible_dirs[1].1.y
        };

        let child_1 = thread::spawn(move || 
          {
            Self::follow_loop(1, pair1_copy, start, adjacent_to_start_pos_1,  start, matrix_ref_1, 1);
          }
        );

        let child_2 = thread::spawn(move || 
          {
            Self::follow_loop(2, pair2_copy, start, adjacent_to_start_pos_2, start, matrix_ref_2, 1);
          }
        );

        loop {

          let (lock1, cvar1) = &*pair1;
          let mut result1 = lock1.lock().unwrap();
          while (*result1).value == 0 {
            result1 = cvar1.wait(result1).unwrap();
            // println!("master read value1: {}", (*result1).value);
          }
          let res1_copy: i32 = (*result1).value;
          (*result1).value = 0; // allow thread to proceed


          let (lock2, cvar2) = &*pair2;
          let mut result2 = lock2.lock().unwrap();
          while (*result2).value == 0 {
            result2 = cvar2.wait(result2).unwrap();
            // println!("master read value2: {}", (*result2).value);
          }
          let res2_copy: i32 = (*result2).value;
          (*result2).value = 0; // allow thread to proceed

          cvar1.notify_one();
          cvar2.notify_one();

          // If same location and same step count, then we have found the midpoint
          if ((*result2).coord == (*result1).coord) && (res1_copy == res2_copy) {
            println!("found solution: {}", res1_copy);
            ans = res1_copy;
            break;
          }

          if res1_copy == -1 || res2_copy == -1 {
            break;
          }

        }

        // let _ = child_1.join();
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

  fn find_start(matrix: &Vec<Vec<char>>) -> Coordinate {
    let mut start: Coordinate = Coordinate {
      x: 0,
      y: 0
    };
    for y in 0..matrix.len() {
      for x in 0..matrix[y].len() {
        if matrix[y][x] == 'S' {
          start.x = x;
          start.y = y;
        }        
      }
    }
    start
  }

  fn find_possible_directions_around_start(start: Coordinate, matrix: &Vec<Vec<char>>) -> Vec::<(char, Coordinate)> {

    let mut possible_starts = Vec::<(char, Coordinate)>::new();

    if let Some(c) = Self::go_north(start) {
      if Self::valid(c, matrix) && vec!['|', '7', 'F'].contains(&matrix[c.y][c.x]) {
        possible_starts.push((matrix[c.y][c.x], c))
      }
    }

    if let Some(c) = Self::go_south(start) {
      if Self::valid(c, matrix) && vec!['|', 'L', 'J'].contains(&matrix[c.y][c.x]) {
        possible_starts.push((matrix[c.y][c.x], c))
      }
    }

    if let Some(c) = Self::go_east(start) {
      if Self::valid(c, matrix) && vec!['-', 'J', '7'].contains(&matrix[c.y][c.x]) {
        possible_starts.push((matrix[c.y][c.x], c))
      }
    }

    if let Some(c) = Self::go_west(start) {
      if Self::valid(c, matrix) && vec!['-', 'L', 'F'].contains(&matrix[c.y][c.x]) {
        possible_starts.push((matrix[c.y][c.x], c))
      }
    }

    possible_starts
  }

  fn follow_loop(thread_num: u8, pair: Arc<(Mutex<Step>, Condvar)>, start_pos: Coordinate, mut current_pos: Coordinate, mut prev_pos: Coordinate, matrix: Arc<Vec<Vec<char>>>, mut count: i32) {
    
    loop {
      let (lock, cvar) = &*pair;
      let mut result = lock.lock().unwrap();
      while (*result).value != 0 {
        result = cvar.wait(result).unwrap();
      }

      let next_pos = Self::next_position(matrix[current_pos.y][current_pos.x], current_pos, prev_pos);
      
      println!("thread {}, next pos - x: {} y: {}", thread_num, next_pos.x, next_pos.y);

      if next_pos == start_pos {
        *result = Step{
          coord: Coordinate {x: next_pos.x, y: next_pos.y},
          value: -1
        };
        cvar.notify_one(); 
        break;
      } else {
        *result = Step{
          coord: Coordinate {x: next_pos.x, y: next_pos.y},
          value: count + 1
        };
        cvar.notify_one();  
        count +=1; 
        prev_pos = current_pos;
        current_pos = next_pos;
      }
    }
  }

  // where is c1 relative to c2?
  fn position(c1: Coordinate, c2: Coordinate) -> Position {
    if c1.x + 1 == c2.x && c1.y == c2.y {
      Position::West
    } else if c1.x == c2.x + 1 && c1.y == c2.y {
      Position::East
    } else if c1.x == c2.x && c1.y + 1 == c2.y {
      Position::North
    } else if c1.x == c2.x && c1.y == c2.y + 1 {
      Position::South
    } else if c1.x + 1 == c2.x && c1.y + 1 == c2.y {
      Position::NorthWest
    } else if c1.x == c2.x + 1 && c1.y + 1 == c2.y {
      Position::NorthEast
    } else if c1.x + 1 == c2.x && c1.y == c2.y + 1 {
      Position::SouthWest
    } else {
      Position::SouthEast
    }
  }

  fn go_east(c: Coordinate) -> Option<Coordinate> {
    Some(
      Coordinate{
        x: c.x + 1,
        y: c.y
      }
    )}

  fn go_west(c: Coordinate) -> Option<Coordinate> {
    if c.x >= 1 {
      Some(
        Coordinate {
          x: c.x - 1,
          y: c.y
        }
      )
    } else { 
      None
    }
  }

  fn go_north(c: Coordinate) -> Option<Coordinate> {
    if c.y >= 1 {
      Some(
        Coordinate{
          x: c.x,
          y: c.y - 1
      })
    } else { 
      None
    }    
  }

  fn go_south(c: Coordinate) -> Option<Coordinate> {
    Some(Coordinate{
      x: c.x,
      y: c.y + 1
    })
  }

  fn valid(c: Coordinate, matrix: &Vec<Vec<char>>) -> bool {
    if c.y >= matrix.len() || c.x >= matrix[0].len() {
      false
    } else {
      true
    }
  }

  fn next_position(symbol: char, symbol_position: Coordinate, prev_position: Coordinate) -> Coordinate 
  {
    let Some(pos) = (match symbol {
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
    }) else {
      panic!("Pipe is taking us out of bounds!!");
    };

    pos
  }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works_on_sample_input() {
        let day10 = Day10::new(&"./inputs/day10_test.input".to_string());
        let AocRes::Int32(res) = day10.part1() else {
            panic!("Failed to get result from part 1!")
        };
        assert!(res == 8);
    }

}