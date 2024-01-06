use regex::Regex;
use crate::aoc::{Aoc, AocRes};
use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub struct Day2 {
    path_to_input: String
}

const RED_LIMIT: i32 = 12;
const GREEN_LIMIT: i32 = 13;
const BLUE_LIMIT: i32 = 14;

impl Aoc for Day2 {

    fn new(path_to_input: &String) -> Self {
        Self{
            path_to_input: path_to_input.clone()
        }
    }

    fn part1(&self) -> AocRes {
        let mut answer: i64 = 0;
        if let Ok(lines) = Self::read_lines(&self.path_to_input) {
            // Consumes the iterator, returns an (Optional) String
            for line in lines {
                if let Ok(ip) = line {
                    // Break via delimiter to separate game from rounds
                    let parts = ip.split(": ");
                    let mut v = parts.collect::<VecDeque<&str>>();
                    let Some(game_id) = v.pop_front() else {
                        panic!("Line does not contain game xx: format!");
                    };
                    let re = Regex::new(r"(\d+)").unwrap();
                    let Some(caps) = re.captures(game_id) else {
                        panic!("Line does not contain any numbers!");
                    };
                    let id = caps[0].to_string();
                    // println!("id: {}", id);
                    let Some(rounds_text) = v.pop_back() else {
                        panic!("Line does not contain round information!");
                    };
                    // Break via delimiter again to separate rounds
                    let rounds = rounds_text.split("; ");
                    let mut game_okay = true;
                    for round in rounds {
                        // Split by color
                        let colors = ["red", "green", "blue"];
                        let mut parsed_round: Vec<(String, i32)> = Vec::<(String, i32)>::new();
                        for color in colors {
                            let formatted = format!(r"(\d+)[^\d]+{}", color);
                            let re = Regex::new(formatted.as_str()).unwrap();
                            let color_and_num = match re.captures(round) {
                                Some(num) => {
                                    // println!("num[1]: {}", num[1].to_string());
                                    (
                                        color.to_string(),
                                        num[1].to_string().parse::<i32>().unwrap(),
                                    )
                                }
                                _ => (color.to_string(), 0),
                            };
                            parsed_round.push(color_and_num);
                        }
                        if Self::check_round_ok(parsed_round) == false {
                            game_okay = false;
                            break;
                        }
                    }
                    if game_okay {
                        answer = answer + id.parse::<i64>().unwrap();
                    }
                }
            }
        }

        AocRes::Int64(answer)
    }

    fn part2(&self) -> AocRes {
       AocRes::Int64(0)
       // TODO: complete this at some point...
    }
}

impl Day2 {

    fn check_round_ok(parsed_round: Vec<(String, i32)>) -> bool {
        for (color, num) in parsed_round {
            match color.as_str() {
                "red" => {
                    if num > RED_LIMIT {
                        return false;
                    }
                }
                "green" => {
                    if num > GREEN_LIMIT {
                        return false;
                    }
                }
                "blue" => {
                    if num > BLUE_LIMIT {
                        return false;
                    }
                }
                _ => return false,
            };
        }

        true
    }

    // The output is wrapped in a Result to allow matching on errors
    // Returns an Iterator to the Reader of the lines of the file.
    fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where
        P: AsRef<Path>,
    {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }
}
