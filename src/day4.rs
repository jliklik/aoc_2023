use crate::aoc::{Aoc, AocRes};
use regex::Regex;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub struct Day4 {
    path_to_input: String
}

impl Aoc for Day4 {

    fn new(path_to_input: &String) -> Self {
        Self {
            path_to_input: path_to_input.clone()
        }
    }

    fn part1(&self) -> AocRes {
        let mut answer: i32 = 0;
        if let Ok(lines) = Self::read_lines(&self.path_to_input) {
            for line in lines {
                if let Ok(ip) = line {
                    let parts = ip.split(": ");
                    let mut v = parts.collect::<VecDeque<&str>>();
                    let Some(id_string) = v.pop_front() else {
                        panic!("Line does not contain game xx: format!");
                    };
                    let re = Regex::new(r"(\d+)").unwrap();
                    let Some(caps) = re.captures(id_string) else {
                        panic!("Line does not contain any numbers!");
                    };
                    let id = caps[0].to_string();
                    let Some(card_text) = v.pop_back() else {
                        panic!("Line does not contain round information!");
                    };
                    let numbers = card_text.split(" | ");
                    let mut n = numbers.collect::<VecDeque<&str>>();
                    let Some(winning_numbers_txt) = n.pop_front() else {
                        panic!("Line does not contain winning number info!");
                    };
                    let Some(numbers_i_have_txt) = n.pop_front() else {
                        panic!("Line does not contain info about numbers you have!");
                    };
                    let re = Regex::new(r"\d+").unwrap();
                    let mut winning_numbers = HashSet::<i32>::new();
                    let mut numbers_i_have = Vec::<i32>::new();
                    for winning_number_match in re.find_iter(winning_numbers_txt) {
                        winning_numbers.insert(
                            winning_number_match
                                .as_str()
                                .to_string()
                                .parse::<i32>()
                                .unwrap(),
                        );
                    }
                    for number_match in re.find_iter(numbers_i_have_txt) {
                        numbers_i_have
                            .push(number_match.as_str().to_string().parse::<i32>().unwrap());
                    }
                    let points = numbers_i_have.iter().fold(0, |acc, x| {
                        if winning_numbers.contains(x) {
                            if acc == 0 {
                                1
                            } else {
                                acc << 1
                            }
                        } else {
                            acc
                        }
                    });
                    // println!("card id: {}, points: {}", id, points);
                    answer = answer + points;
                }
            }
        }
        AocRes::Int32(answer)
    }

    fn part2(&self) -> AocRes {
        let mut answer: i32 = 0;
        let mut num_copies_per_card = HashMap::<i32, i32>::new();
        let mut total_cards = 1;
        if let Ok(lines) = Self::read_lines(&self.path_to_input) {
            for line in lines {
                if let Ok(ip) = line {
                    total_cards = total_cards + 1;
                    let parts = ip.split(": ");
                    let mut v = parts.collect::<VecDeque<&str>>();
                    let Some(id_string) = v.pop_front() else {
                        panic!("Line does not contain game xx: format!");
                    };
                    let re = Regex::new(r"(\d+)").unwrap();
                    let Some(caps) = re.captures(id_string) else {
                        panic!("Line does not contain any numbers!");
                    };
                    let id = caps[0].to_string().parse::<i32>().unwrap();
                    let Some(card_text) = v.pop_back() else {
                        panic!("Line does not contain round information!");
                    };
                    let numbers = card_text.split(" | ");
                    let mut n = numbers.collect::<VecDeque<&str>>();
                    let Some(winning_numbers_txt) = n.pop_front() else {
                        panic!("Line does not contain winning number info!");
                    };
                    let Some(numbers_i_have_txt) = n.pop_front() else {
                        panic!("Line does not contain info about numbers you have!");
                    };
                    let re = Regex::new(r"\d+").unwrap();
                    let mut winning_numbers = HashSet::<i32>::new();
                    let mut numbers_i_have = Vec::<i32>::new();
                    for winning_number_match in re.find_iter(winning_numbers_txt) {
                        winning_numbers.insert(
                            winning_number_match
                                .as_str()
                                .to_string()
                                .parse::<i32>()
                                .unwrap(),
                        );
                    }
                    for number_match in re.find_iter(numbers_i_have_txt) {
                        numbers_i_have
                            .push(number_match.as_str().to_string().parse::<i32>().unwrap());
                    }
                    let mut existing_copies_of_this_card = match num_copies_per_card.get(&id) {
                        Some(&num_cards) => num_cards,
                        _ => 0,
                    };
                    existing_copies_of_this_card = existing_copies_of_this_card + 1;
                    // each card has at least the original copy
                    num_copies_per_card.insert(id, existing_copies_of_this_card);
                    numbers_i_have.iter().fold(id, |acc, x| {
                        if winning_numbers.contains(x) {
                            let win_new_card_id = acc + 1;
                            let mut existing_copies_of_next_card =
                                match num_copies_per_card.get(&win_new_card_id) {
                                    Some(&num_cards) => num_cards,
                                    _ => 0,
                                };
                            existing_copies_of_next_card =
                                existing_copies_of_next_card + existing_copies_of_this_card;
                            num_copies_per_card
                                .insert(win_new_card_id, existing_copies_of_next_card);
                            win_new_card_id
                        } else {
                            acc
                        }
                    });
                }
            }

            // count number of cards obtained
            for card_id in 1..total_cards {
                let num_copies = match num_copies_per_card.get(&card_id) {
                    Some(&cards) => cards,
                    None => 0,
                };
                // println!{"card {} has {} copies", card_id, num_copies};
                answer = answer + num_copies;
            }
        }

        AocRes::Int32(answer)
    }

}

impl Day4 {
   
    fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where
        P: AsRef<Path>,
    {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }
}
