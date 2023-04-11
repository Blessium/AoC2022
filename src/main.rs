use crate::resolver::Resolver;

pub mod days;
pub mod resolver;

fn main() {
    Resolver::new(days::day6::Day6::new()).get_output();
}
