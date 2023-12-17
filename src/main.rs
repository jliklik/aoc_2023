mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

use day1::Day1;
use day2::Day2;
use day3::Day3;
use day4::Day4;
use day5::Day5;
use day6::Day6;

fn main() {
    let aoc1 = Day1::new("./inputs/day1.input".to_string());
    println!("day1 - part1: {}", aoc1.part1);
    println!("day1 - part2: {}", aoc1.part2);

    let aoc2 = Day2::new("./inputs/day2.input".to_string());
    println!("day2 - part1: {}", aoc2.part1);
    println!("day2 - part2: {}", aoc2.part2);

    let aoc3 = Day3::new("./inputs/day3.input".to_string());
    println!("day3 - part1: {}", aoc3.part1);
    println!("day3 - part2: {}", aoc3.part2);

    let aoc4 = Day4::new("./inputs/day4.input".to_string());
    println!("day4 - part1: {}", aoc4.part1);
    println!("day4 - part2: {}", aoc4.part2);

    let aoc5 = Day5::new("./inputs/day5.input".to_string());
    println!("day5 - part1: {}", aoc5.part1);
    println!("day5 - part2: {}", aoc5.part2);

    let aoc6 = Day6::new("./inputs/day6.input".to_string());
    println!("day6 - part1: {}", aoc6.part1);
    println!("day6 - part2: {}", aoc6.part2);
}
