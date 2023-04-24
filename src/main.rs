use crate::resolver::Resolver;

pub mod days;
pub mod resolver;

fn main() {
    Resolver::new(days::day12::Day12::new()).get_output();
}
