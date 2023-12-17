use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::VecDeque;

// Algo:
// Parse each 5 card combo
// Sort in this order AKQ... etc.
// Find largest matching group
// - Parse left to right, keep track of largest group in a priority queue
// - This will find 5 of a kind, 4 of a kind, 3 of a kind, full house, two pair, one pair combos and high card


pub struct Day7 {
  pub part1: i32,
  pub part2: i32
}

#[derive(Eq, PartialEq, PartialOrd, Ord, Clone, Copy)]
enum HandTypes {
  Unknown,
  FiveOfAKind(u8),
  FourOfAKind(u8, u8),
  FullHouse(u8, u8),
  ThreeOfAKind(u8, u8, u8),
  TwoPair(u8, u8, u8),
  OnePair(u8, u8, u8, u8),
  HighCard(u8, u8, u8, u8, u8)
}

impl Day7 {

  pub fn new<P>(path_to_input: P) -> Self 
  where P: AsRef<Path>, {
    Self {
      part1: Self::part1(&path_to_input),
      part2: Self::part2(&path_to_input)
    }
  }

  fn char_to_val_part1(c: &char) -> u8 {
    match c {
      '2' => 2,
      '3' => 3,
      '4' => 4,
      '5' => 5,
      '6' => 6,
      '7' => 7,
      '8' => 8,
      '9' => 9,
      'T' => 10,
      'J' => 11,
      'Q' => 12,
      'K' => 13,
      'A' => 14,
      _ => 0
    }
  }

  fn char_to_val_part2(c: &char) -> u8 {
    match c {
      '2' => 2,
      '3' => 3,
      '4' => 4,
      '5' => 5,
      '6' => 6,
      '7' => 7,
      '8' => 8,
      '9' => 9,
      'T' => 10,
      'J' => 1,
      'Q' => 12,
      'K' => 13,
      'A' => 14,
      _ => 0
    }
  }

  fn hand_type_to_val(hand_type: HandTypes) -> u8 {
    match hand_type {
      HandTypes::FiveOfAKind(_) => std::u8::MAX,
      HandTypes::FourOfAKind(_, _) => std::u8::MAX - 1,
      HandTypes::FullHouse(_, _) => std::u8::MAX - 2,
      HandTypes::ThreeOfAKind(_, _, _) => std::u8::MAX - 3,
      HandTypes::TwoPair(_, _, _) => std::u8::MAX - 4,
      HandTypes::OnePair(_, _, _, _) => std::u8::MAX - 5,
      HandTypes::HighCard(_, _, _, _, _) => std::u8::MAX - 6,
      HandTypes::Unknown => 0
    }
  }

  fn hand_type_to_comparator(hand_type: HandTypes) -> (u8, u8, u8, u8, u8) {

    match hand_type {
      HandTypes::Unknown => (0, 0, 0, 0, 0),
      HandTypes::HighCard(c1, c2, c3, c4, c5) => (c1, c2, c3, c4, c5),
      HandTypes::OnePair(c1, c2, c3, c4) => {
        (Self::hand_type_to_val(hand_type), c1, c2, c3, c4)
      }
      HandTypes::TwoPair(c1, c2, c3) => {
        (Self::hand_type_to_val(hand_type), std::u8::MAX, c1, c2, c3)
      }
      HandTypes::ThreeOfAKind(c1, c2, c3) => {
        (Self::hand_type_to_val(hand_type), std::u8::MAX, c1, c2, c3)
      }
      HandTypes::FullHouse(c1, c2) => {
        (Self::hand_type_to_val(hand_type), std::u8::MAX, std::u8::MAX, c1, c2)
      }
      HandTypes::FourOfAKind(c1, c2) => {
        (Self::hand_type_to_val(hand_type), std::u8::MAX, std::u8::MAX, c1, c2)
      }
      HandTypes::FiveOfAKind(c1) => {
        (Self::hand_type_to_val(hand_type), std::u8::MAX, std::u8::MAX, std::u8::MAX, c1)
      }
    }
  }

  fn find_repeats_part1(mut sorted: Vec<char>) -> Vec<(u8, u8)> {
    let mut count = 1;
    let mut char_counts = Vec::new();
    let l = sorted.len();
    let mut curr = sorted.pop();
    let mut next = curr.clone();
    for i in 0..l {
      match curr {
        Some(c1) => {
          next = sorted.pop();
          match next {
            Some(c2) => {
              if c1 == c2 {
                count = count + 1
              } else {
                // println!("sorted: {}, count: {} char: {}", 
                //   sorted.clone().into_iter().collect::<String>(), count, c1);
                char_counts.push((count, Self::char_to_val_part1(&c1)));
                count = 1;
              }
            }
            None => {
              // println!("sorted: {}, count: {} char: {}", 
              //   sorted.clone().into_iter().collect::<String>(), count, c1);
              char_counts.push((count, Self::char_to_val_part1(&c1)));
            }
          }
        }
        None => ()
      }
      curr = next
    }

    char_counts.sort_by_key(|&a| (a.0, a.1));
    // dbg!(&char_counts);
    char_counts
  }

  fn find_repeats_part2(mut sorted: Vec<char>) -> Vec<(u8, u8)> {

    let mut joker_count = 0;

    let mut count = 1;
    let mut char_counts = Vec::new();
    let l = sorted.len();
    let mut curr = sorted.pop();
    let mut next = curr.clone();
    for i in 0..l {
      match curr {
        Some(c1) => {
          next = sorted.pop();
          match next {
            Some(c2) => {
              if c1 == c2 {
                count = count + 1
              } else {
                // println!("sorted: {}, count: {} char: {}", 
                //   sorted.clone().into_iter().collect::<String>(), count, c1);
                if c1 == 'J' {
                  joker_count = count;
                } else {
                  char_counts.push((count, Self::char_to_val_part1(&c1)));
                }
                count = 1;
              }
            }
            None => {
              // println!("sorted: {}, count: {} char: {}", 
              //   sorted.clone().into_iter().collect::<String>(), count, c1);
              if c1 == 'J' {
                joker_count = count;
              } else {
                char_counts.push((count, Self::char_to_val_part1(&c1)));
              }
            }
          }
        }
        None => ()
      }
      curr = next
    }

    if char_counts.len() == 0 && joker_count == 5 {
      // edge case where all jokers - then no values pushed
      char_counts.push((joker_count, Self::char_to_val_part1(&'J')));
    } else {
      char_counts.sort_by_key(|&(count, char)| (count, char));
      // Add joker count to the first elem count
      char_counts.reverse();
      let (top_count, top_value_char) = char_counts[0];
      std::mem::replace(&mut char_counts[0], (top_count + joker_count, top_value_char));
      char_counts.reverse();
    }
    
    dbg!(&char_counts);
    char_counts
  }

  fn parse_char_counts(mut char_counts: Vec<(u8, u8)>) -> HandTypes {
    let hand_type = match char_counts.pop() {
      Some((5, c1)) => {
        println!("5 of a kind!");
        HandTypes::FiveOfAKind(c1)
      }
      Some((4, c1)) => {
        let Some((1, c2)) = char_counts.pop() else {
          panic!("4 of a kind - Last card not found!");
        };
        println!("4 of a kind!");
        HandTypes::FourOfAKind(c1, c2)
      }
      Some((3, c1)) => {
        match char_counts.pop() {
          Some((2, c2)) => {
            HandTypes::FullHouse(c1, c2)
          },
          Some((1, c2)) => {
            let Some((1, c3)) = char_counts.pop() else {
             panic!("Three of a kind - 5th card not found!")
            };
            HandTypes::ThreeOfAKind(c1, c2, c3)
          },
          _ => HandTypes::Unknown
        }
      }
      Some((2, c1)) => {
        match char_counts.pop() {
          Some((2, c2)) => {
            let Some((1, c3)) = char_counts.pop() else {
              panic!("Two pair - Last card not found!");
            };
            println!("2 pair");
            HandTypes::TwoPair(c1, c2, c3)
          },
          Some((1, c2)) => {
            let Some((1, c3)) = char_counts.pop() else {
              panic!("Two pair - 3rd card not found!");
            };
            let Some((1, c4)) = char_counts.pop() else {
              panic!("Two pair - 4th card not found!");
            };
            println!("1 pair");
            HandTypes::OnePair(c1, c2, c3, c4)
          },
          _ => HandTypes::Unknown
        }
      }
      Some((1, c1)) => {
        let Some((1, c2)) = char_counts.pop() else {
          panic!("High card - 2nd card not found!");
        };
        let Some((1, c3)) = char_counts.pop() else {
          panic!("High card - 3rd card not found!");
        };
        let Some((1, c4)) = char_counts.pop() else {
          panic!("High card - 4th card not found!");
        };
        let Some((1, c5)) = char_counts.pop() else {
          panic!("High card - 5th card not found!");
        };
        println!("high card");
        HandTypes::HighCard(c1, c2, c3, c4, c5)
      }
      _ => {
        println!("unknown");
        HandTypes::Unknown  
      }   
    };
    hand_type
  }

  fn categorize_type_part1(hand: &str) -> (HandTypes, String) {
    let mut chars: Vec<char> = hand.chars().collect();
    chars.sort_by(|a, b| Self::char_to_val_part1(a).cmp(&(Self::char_to_val_part1(b))));
    let sorted = chars.clone().into_iter().collect::<String>();
    let mut char_counts: Vec<(u8, u8)> = Self::find_repeats_part1(chars.clone());
    let hand_type = Self::parse_char_counts(char_counts);
    // type, typedata, hand, bid
    (hand_type, sorted)
  }

  fn categorize_type_part2(hand: &str) -> (HandTypes, String) {
    let mut chars: Vec<char> = hand.chars().collect();
    chars.sort_by(|a, b| Self::char_to_val_part1(a).cmp(&(Self::char_to_val_part1(b))));
    let sorted = chars.clone().into_iter().collect::<String>();
    let mut char_counts: Vec<(u8, u8)> = Self::find_repeats_part2(chars.clone());
    let hand_type = Self::parse_char_counts(char_counts);
    // type, typedata, hand, bid
    (hand_type, sorted)
  }

  fn part1<P>(path_to_input: P) -> i32
  where P: AsRef<Path>, {
    let mut total_sum = 0;
    let mut hands = Vec::<(HandTypes, i32, String)>::new();
    
    if let Ok(lines) = Self::read_lines(path_to_input) {
      // Consumes the iterator, returns an (Optional) String
      for line in lines {
        if let Ok(l) = line {
          let mut v = l.split(" ").collect::<VecDeque<&str>>();
          let Some(bid) = v.pop_back() else {
            panic!("Could not parse bid!");
          };
          let bid = bid.to_string().parse::<i32>().unwrap();
          let Some(hand) = v.pop_front() else {
            panic!("Could not parse hand!");
          };
          let (hand_type, sorted) = Self::categorize_type_part1(hand);
          // hands.push((hand_type, bid, sorted));
          hands.push((hand_type, bid, hand.to_string()));
        }
      }

    }

    //hands.sort_unstable_by_key(|h| Self::hand_type_to_comparator(h.0));
    hands.sort_unstable_by_key(|h| 
      {
        let v = h.2.chars().collect::<Vec<char>>();
        let (c1, c2, c3, c4, c5) = (v[0], v[1], v[2], v[3], v[4]);
        (
          Self::hand_type_to_val(h.0), 
          Self::char_to_val_part1(&c1), 
          Self::char_to_val_part1(&c2), 
          Self::char_to_val_part1(&c3), 
          Self::char_to_val_part1(&c4), 
          Self::char_to_val_part1(&c5)
        )
      }
    );
    for h in &hands {
      println!("hand: {}, bid: {}", h.2, h.1);
    }
    let (total_ranks, total_sum) = hands.iter().fold((1, 0), |(rank, sum), (handtype, bid, hand) | {
      (rank + 1, sum + rank * bid)
    });
    println!("total_sum: {}", total_sum);

    total_sum

  }

  fn part2<P>(path_to_input: P) -> i32
  where P: AsRef<Path>, {
    let mut total_sum = 0;
    let mut hands = Vec::<(HandTypes, i32, String)>::new();
    
    if let Ok(lines) = Self::read_lines(path_to_input) {
      // Consumes the iterator, returns an (Optional) String
      for line in lines {
        if let Ok(l) = line {
          let mut v = l.split(" ").collect::<VecDeque<&str>>();
          let Some(bid) = v.pop_back() else {
            panic!("Could not parse bid!");
          };
          let bid = bid.to_string().parse::<i32>().unwrap();
          let Some(hand) = v.pop_front() else {
            panic!("Could not parse hand!");
          };
          let (hand_type, sorted) = Self::categorize_type_part2(hand);
          // hands.push((hand_type, bid, sorted));
          hands.push((hand_type, bid, hand.to_string()));
        }
      }
    }

    //hands.sort_unstable_by_key(|h| Self::hand_type_to_comparator(h.0));
    hands.sort_unstable_by_key(|h| 
      {
        let v = h.2.chars().collect::<Vec<char>>();
        let (c1, c2, c3, c4, c5) = (v[0], v[1], v[2], v[3], v[4]);
        (
          Self::hand_type_to_val(h.0), 
          Self::char_to_val_part2(&c1), 
          Self::char_to_val_part2(&c2), 
          Self::char_to_val_part2(&c3), 
          Self::char_to_val_part2(&c4), 
          Self::char_to_val_part2(&c5)
        )
      }
    );
    for h in &hands {
      println!("hand: {}, bid: {}", h.2, h.1);
    }
    let (total_ranks, total_sum) = hands.iter().fold((1, 0), |(rank, sum), (handtype, bid, hand) | {
      (rank + 1, sum + rank * bid)
    });
    println!("total_sum: {}", total_sum);

    total_sum
  }

  fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
  where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
  }

}