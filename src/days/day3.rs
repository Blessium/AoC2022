use crate::resolver::Day;
use std::collections::HashSet;

pub struct Day3;

impl Day3 {
    pub fn new() -> Self {
        Day3 {}
    }
}

pub fn get_priority(item: char) -> i32 {
    if item.is_uppercase() {
        item as i32 - 38
    } else {
        item as i32 - 96
    }
}


pub fn get_diff_chars(first: &str, second: &str) -> Vec<char> {
    let first_set: HashSet<char> = first.chars().collect();
    let second_set: HashSet<char> = second.chars().collect();
    (&first_set & &second_set).into_iter().collect()
}


impl Day for Day3 {
    fn solve(&self) -> String {
        let input = self.get_input("input3");
        let mut lines = input.split("\n").collect::<Vec<&str>>();
        lines.remove(lines.len() - 1);

        let s = lines.into_iter()
            .map(|line| {
                let (first, second) = line.split_at(line.len() / 2);
                let chars = get_diff_chars(first, second);
                chars.into_iter().map(|c| get_priority(c)).collect::<Vec<i32>>().iter().sum() 
            }).collect::<Vec<i32>>();

        s.iter().sum::<i32>().to_string()
    }
}
