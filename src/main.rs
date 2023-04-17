use crate::resolver::Resolver;

pub mod days;
pub mod resolver;

fn main() {
    Resolver::new(days::day7::Day7::new()).get_output();
}
