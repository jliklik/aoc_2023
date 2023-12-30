use std::path::Path;
pub trait Aoc<P, N>
where
    P: AsRef<Path>,
    N: num_traits::Num,
{
    fn part1(path_to_input: P) -> N;
    fn part2(path_to_input: P) -> N;
}

// If a trait takes in generic type parameters, then it cannot be made into a trait object
// So might have to use concrete types here like String and i64 for it to work...
// Then we can have a vector of aoc objects and iterate over them by calling part1 and part2