// v = charge_time (c)
// d = v(Total_time (T) - charge_time)

// d = charge_time * (Total_time - charge_time)
// d = -c^2 + c*T

// ^ d
// |    _ _
// |   /   \
// |  /     \
// |------------ <- find all combinations beyond this threshold D
// |/         \
// |-------------> c

use crate::aoc::{Aoc, AocRes};
use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};

pub struct Day6 {
    path_to_input: String
}

impl Aoc for Day6 {
    fn new(path_to_input: &String) -> Self {
        Self {
            path_to_input: path_to_input.clone()
        }
    }

    fn part1(&self) -> AocRes {
        let file = File::open(&self.path_to_input).unwrap();
        let mut buffer = io::BufReader::new(file);
        let mut first_line = String::new();
        let _ = buffer.read_line(&mut first_line);

        let re = Regex::new(r"\d+").unwrap();
        let mut total_times = Vec::<i32>::new();
        for time_match in re.find_iter(first_line.as_str()) {
            total_times.push(time_match.as_str().to_string().parse::<i32>().unwrap());
        }

        let mut second_line = String::new();
        let _ = buffer.read_line(&mut second_line);

        let mut distances_to_beat = Vec::<i32>::new();
        for dist_match in re.find_iter(second_line.as_str()) {
            distances_to_beat.push(dist_match.as_str().to_string().parse::<i32>().unwrap());
        }

        let times_and_distances = std::iter::zip(total_times, distances_to_beat);
        let mut answer = 1;
        for (total_time, dist) in times_and_distances {
            let mut winning_combinations = 0;
            for c in 0..total_time {
                let d = -1 * c * c + c * total_time;
                if d > dist {
                    winning_combinations = winning_combinations + 1;
                }
            }
            // println!("total_time: {total_time}, dist to beat: {dist} winning_combinations: {winning_combinations}");
            answer = answer * winning_combinations;
        }

        AocRes::Int32(answer)
    }

    fn part2(&self) -> AocRes {
        let file = File::open(&self.path_to_input).unwrap();
        let mut buffer = io::BufReader::new(file);
        let mut first_line = String::new();
        let _ = buffer.read_line(&mut first_line);

        let re = Regex::new(r"\d+").unwrap();
        let mut total_times = Vec::<String>::new();
        for time_match in re.find_iter(first_line.as_str()) {
            total_times.push(time_match.as_str().to_string());
        }

        let mut second_line = String::new();
        let _ = buffer.read_line(&mut second_line);

        let mut distances_to_beat = Vec::<String>::new();
        for dist_match in re.find_iter(second_line.as_str()) {
            distances_to_beat.push(dist_match.as_str().to_string());
        }

        let total_time = total_times
            .iter()
            .fold("".to_string(), |acc, el| acc + el)
            .parse::<i64>()
            .unwrap();
        let distance_to_beat = distances_to_beat
            .iter()
            .fold("".to_string(), |acc, el| acc + el)
            .parse::<i64>()
            .unwrap();

        let mut winning_combinations = 0;
        for c in 0..total_time {
            let d = -1 * c * c + c * total_time;
            if d > distance_to_beat {
                winning_combinations = winning_combinations + 1;
            }
        }
        //println!("total_time: {total_time}, dist to beat: {distance_to_beat} winning_combinations: {winning_combinations}");

        AocRes::Int32(winning_combinations)
    }
}

