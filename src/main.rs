mod day1;
mod day2;

use day1::Day1;
use day2::Day2;

fn main() {
    let aoc1 = Day1::new("./inputs/day1.input".to_string());
    println!("day1 - part1: {}", aoc1.part1);
    println!("day1 - part2: {}", aoc1.part2);

    let aoc2 = Day2::new("./inputs/day2.input".to_string());
    println!("day2 - part1: {}", aoc2.part1);
    println!("day2 - part2: {}", aoc2.part2);
}
