use crate::resolver::Resolver;

pub mod days;
pub mod resolver;

fn main() {
    Resolver::new(days::day11::Day11::new()).get_output();
}
