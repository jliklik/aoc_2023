use crate::aoc::{Aoc, AocRes};
use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub struct Day3 {
    path_to_input: String
}

impl Aoc for Day3 {

    fn new(path_to_input: &String) -> Self {
        Self {
            path_to_input: path_to_input.clone()
        }
    }

    fn part1(&self) -> AocRes {
        let mut answer: i32 = 0;
        let mat = Self::read_into_matrix(&self.path_to_input);
        let m = mat.len();
        let n = mat[0].len();
        // println!("rows: {}, cols: {}", m, n);
        // Assumption: rows are of consistent length
        for i in 0..m {
            let row = mat[i].clone();
            let next_row = if i >= (m - 1) {
                vec!['.'; n]
            } else {
                mat[i + 1].clone()
            };
            let prev_row = if i <= 0 {
                vec!['.'; n]
            } else {
                mat[i - 1].clone()
            };
            // match all numbers and get their positions
            let s: String = row.clone().into_iter().collect();
            let re = Regex::new(r"\d+").unwrap();
            for found_match in re.find_iter(s.as_str()) {
                let mut sidx = found_match.start();
                let mut eidx = found_match.end();
                // look around for a symbol
                // it would be more efficient to match symbols first then look around for numbers, but harder to do
                sidx = if ((sidx as i32) - 1) < 0 {
                    sidx
                } else {
                    sidx - 1
                };
                eidx = if eidx + 1 > (n - 1) { eidx } else { eidx + 1 };
                if prev_row[sidx..eidx].iter().any(|el| {
                    !matches!(
                        el,
                        '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '0' | '.'
                    )
                }) || row[sidx..eidx].iter().any(|el| {
                    !matches!(
                        el,
                        '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '0' | '.'
                    )
                }) || next_row[sidx..eidx].iter().any(|el| {
                    !matches!(
                        el,
                        '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '0' | '.'
                    )
                }) {
                    let num = found_match.as_str().to_string().parse::<i32>().unwrap();
                    // if i == 0 || i == (m-1) {
                    //   println!("{}", num);
                    // }
                    answer = answer + num;
                }
            }
        }
        AocRes::Int32(answer)
    }

    fn part2(&self) -> AocRes {
        let mut answer: i32 = 0;
        AocRes::Int32(answer)
    }


}

impl Day3 {

    fn read_into_matrix<P>(path_to_input: P) -> Vec<Vec<char>>
    where
        P: AsRef<Path>,
    {
        if let Ok(lines) = Self::read_lines(path_to_input) {
            let mut mat: Vec<Vec<char>> = Vec::new();
            for line in lines {
                if let Ok(l) = line {
                    let chars: Vec<_> = l.chars().collect();
                    mat.push(chars);
                }
            }
            mat
        } else {
            panic!("not able to read file!")
        }
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
