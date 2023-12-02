mod day1;

use day1::Day1;

fn main() {
    let aoc1 = Day1::new("./inputs/day1.input".to_string());
    println!("day1 - part1: {}", aoc1.part1);
    println!("day1 - part2: {}", aoc1.part2);
}
