use std::fmt;

pub enum AocRes {
    Int32(i32),
    Int64(i64),
    UInt32(u32),
    UInt64(u64)
}

impl fmt::Display for AocRes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AocRes::Int32(data) => write!(f, "{}", data),
            AocRes::Int64(data) => write!(f, "{}", data),
            AocRes::UInt32(data) => write!(f, "{}", data),
            AocRes::UInt64(data) => write!(f, "{}", data),
        }
    }
}

pub trait Aoc
{
    fn new<'a>(path_to_input: &'a String) -> Self where Self: Sized;
    fn part1(&self) -> AocRes;
    fn part2(&self) -> AocRes;
}

// If a trait takes in generic type parameters, then it cannot be made into a trait object
// So might have to use concrete types here like String and i64 for it to work...
// Then we can have a vector of aoc objects and iterate over them by calling part1 and part2