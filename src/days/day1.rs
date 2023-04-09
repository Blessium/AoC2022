use crate::resolver::{Day, Resolver};

pub struct Day1 {
    
}

impl Day1 {
    pub fn new() -> Self {
        Day1 {
        }
    }
}

impl Day for Day1 {
    fn solve(&self) -> &str {
        let something = self.get_input("input1");
        "day1"
    }
}
