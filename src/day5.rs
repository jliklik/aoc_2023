use crate::aoc::{Aoc, AocRes};
use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::sync::mpsc::channel;
use std::thread::spawn;

pub struct Day5 {
    path_to_input: String
}

impl Aoc for Day5 {

    fn new(path_to_input: &String) -> Self {
        Self {
            path_to_input: path_to_input.clone()
        }
    }

    fn part1(&self) -> AocRes {
        let mut maps = VecDeque::<VecDeque<(u64, u64, u64)>>::new();
        maps.push_back(Self::parse_ranges(&self.path_to_input, "seed-to-soil"));
        maps.push_back(Self::parse_ranges(&self.path_to_input, "soil-to-fertilizer"));
        maps.push_back(Self::parse_ranges(&self.path_to_input, "fertilizer-to-water"));
        maps.push_back(Self::parse_ranges(&self.path_to_input, "water-to-light"));
        maps.push_back(Self::parse_ranges(&self.path_to_input, "light-to-temperature"));
        maps.push_back(Self::parse_ranges(
            &self.path_to_input,
            "temperature-to-humidity",
        ));
        maps.push_back(Self::parse_ranges(&self.path_to_input, "humidity-to-location"));

        let (sender, receiver) = channel();
        let mut handle_vec = vec![];

        let seeds = Self::get_seeds(&self.path_to_input);
        let mut answers = Vec::<u64>::new();

        for seed in seeds {
            let sender_clone = sender.clone();
            let maps_clone = maps.clone();
            let handle = spawn(move || {
                // make a handle
                let loc = Self::apply_chain(seed, &maps_clone);
                sender_clone.send(loc).unwrap(); // use the sender_clone to send the work to the receiver
            });
            handle_vec.push(handle);
        }

        for handle in handle_vec {
            // stop until the threads are done
            handle.join().unwrap();
        }

        while let Ok(res) = receiver.try_recv() {
            // println!("thread res: {}", res);
            answers.push(res); // push the results from receiver.recv() into the vec
        }

        let Some(mut answer) = answers.pop() else {
            panic!("No results!")
        };
        for a in answers {
            if a < answer {
                answer = a;
            }
        }

        AocRes::UInt64(answer)
    }

    fn part2(&self) -> AocRes {
        let mut answer: u64 = 0;
        AocRes::UInt64(answer)
    }
}

impl Day5 {

    fn apply_chain(seed: u64, maps: &VecDeque<VecDeque<(u64, u64, u64)>>) -> u64 {
        maps.iter()
            .fold(seed, |acc, map| Self::find_mapped_value(acc, map))
    }

    fn find_mapped_value(val1: u64, map: &VecDeque<(u64, u64, u64)>) -> u64 {
        let ret = map
            .iter()
            .fold(val1, |acc, (val1_start, range, val2_start)| {
                if (val1 >= *val1_start) && (val1 <= (val1_start + range)) {
                    let diff = val1 - val1_start;
                    val2_start + diff
                } else {
                    acc
                }
            });
        // println!("{ret}");
        ret
    }

    fn get_seeds<P>(filename: &P) -> VecDeque<u64>
    where
        P: AsRef<Path>,
    {
        let file = File::open(filename).unwrap();
        let mut buffer = io::BufReader::new(file);
        let mut first_line = String::new();
        let _ = buffer.read_line(&mut first_line);
        let Some(seeds) = first_line
            .split(": ")
            .collect::<VecDeque<&str>>()
            .pop_back()
        else {
            panic!("No seeds!");
        };
        let seeds = seeds.split(" ").collect::<VecDeque<&str>>();
        seeds
            .into_iter()
            .map(|s| {
                let f = s.to_string().chars().fold("".to_string(), |acc, c| {
                    if "1234567890".contains(c) {
                        acc + &c.to_string()
                    } else {
                        acc
                    }
                });
                f.parse::<u64>().unwrap()
            })
            .collect()
    }

    // parses into a hashmap
    fn parse_ranges<P>(path_to_input: &P, map_name: &str) -> VecDeque<(u64, u64, u64)>
    where
        P: AsRef<Path>,
    {
        let mut bins = VecDeque::<(u64, u64, u64)>::new();
        let mut start_parsing = false;
        if let Ok(lines) = Self::read_lines(path_to_input) {
            for line in lines {
                if let Ok(l) = line {
                    if l.contains(map_name) {
                        start_parsing = true;
                        continue;
                    } else if l == "".to_string() && start_parsing == true {
                        break;
                    }
                    if start_parsing == true {
                        let numbers = l.split(" ").collect::<VecDeque<&str>>();
                        // println!("{}, {}, {}", numbers[0], numbers[1], numbers[2]);
                        let dest = numbers[0].to_string().parse::<u64>().unwrap();
                        let source = numbers[1].to_string().parse::<u64>().unwrap();
                        let range = numbers[2].to_string().parse::<u64>().unwrap();
                        bins.push_back((source, range, dest));
                    }
                }
            }
        }
        bins
    }

    fn read_lines<P>(filename: &P) -> io::Result<io::Lines<io::BufReader<File>>>
    where
        P: AsRef<Path>,
    {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }
}
