use crate::resolver::Resolver;

pub mod days;
pub mod resolver;

fn main() {
    Resolver::new(days::day3::Day3::new()).get_output();
}
