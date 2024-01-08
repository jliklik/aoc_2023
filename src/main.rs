mod aoc;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

use aoc::Aoc;
use day1::Day1;
use day2::Day2;
use day3::Day3;
use day4::Day4;
use day5::Day5;
use day6::Day6;
use day7::Day7;
use day8::Day8;
use day9::Day9;

fn main() {

    let mut aoc = Vec::<Box<dyn Aoc>>::new();

    aoc.push(Box::new(Day1::new(&"./inputs/day1.input".to_string())));
    aoc.push(Box::new(Day2::new(&"./inputs/day2.input".to_string())));
    aoc.push(Box::new(Day3::new(&"./inputs/day3.input".to_string())));
    aoc.push(Box::new(Day4::new(&"./inputs/day4.input".to_string())));
    aoc.push(Box::new(Day5::new(&"./inputs/day5.input".to_string())));
    aoc.push(Box::new(Day6::new(&"./inputs/day6.input".to_string())));
    aoc.push(Box::new(Day7::new(&"./inputs/day7.input".to_string())));
    aoc.push(Box::new(Day8::new(&"./inputs/day8.input".to_string())));
    aoc.push(Box::new(Day9::new(&"./inputs/day9.input".to_string())));

    aoc.into_iter().fold(1, |acc, el| {
        println!("day{} - part1: {}", acc, el.part1());
        println!("day{} - part2: {}", acc, el.part2());
        acc + 1
    });
}
