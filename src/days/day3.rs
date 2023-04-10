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

pub fn get_diff_generic(elements: Vec<&str>) -> Vec<char>{
    let mut hashes: Vec<HashSet<char>> = vec![];

    for element in elements {
        hashes.push(element.chars().collect());
    }

    let mut result: HashSet<char> = hashes.pop().expect("size is null");

    for hash in hashes {
        result = &result & (&hash);
    }

    result.into_iter().collect::<Vec<char>>()
}

pub fn part1(lines: Vec<&str>) -> i32 {
        let s = lines.into_iter()
            .map(|line| {
                let (first, second) = line.split_at(line.len() / 2);
                let chars = get_diff_chars(first, second);
                chars.into_iter().map(|c| get_priority(c)).collect::<Vec<i32>>().iter().sum() 
            }).collect::<Vec<i32>>();
        s.iter().sum::<i32>()
}

pub fn part2(lines: Vec<&str>) -> i32 {
    let s = lines.into_iter()
        .enumerate()
        .scan(vec![], | current, (index, value)| {
            current.push(value);
            if ((index + 1) % 3 ) == 0 {
                let mut var = get_diff_generic(current.to_vec());
                current.clear();
                Some(var.pop())
            }  else {
                Some(Some('&'))
            }
        }).collect::<Vec<Option<char>>>();
    
    s.into_iter()
        .flatten()
        .filter(|x| x != &'&')
        .map(|x| { 
            get_priority(x)

        })
        .collect::<Vec<i32>>()
        .iter()
        .sum()
}


impl Day for Day3 {
    fn solve(&self) -> String {
        let input = self.get_input("input3");
        let mut lines = input.split("\n").collect::<Vec<&str>>();
        lines.remove(lines.len() - 1);

        part2(lines).to_string()
    }
}
