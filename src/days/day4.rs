use crate::resolver::Day;
use std::ops::RangeInclusive;

pub struct Day4;

impl Day4 {
    pub fn new() -> Self {
        Day4 {}
    }
}

trait CompareRange {
    fn contains_range(&self, other: &RangeInclusive<i32>) -> bool;
    fn has_common_elements_with(&self, other: &RangeInclusive<i32>) -> bool;
}

impl CompareRange for RangeInclusive<i32> {
    fn contains_range(&self, other: &RangeInclusive<i32>) -> bool {
        self.contains(other.start()) && self.contains(other.end())
    }
    fn has_common_elements_with(&self, other: &RangeInclusive<i32>) -> bool {
        self.contains(other.start()) || self.contains(other.end())
    }
}

fn get_range_from_element(element: &str) -> RangeInclusive<i32> {
    let element_splitted: Vec<&str> = element.split("-").collect();
    RangeInclusive::new(
        element_splitted[0].parse::<i32>().unwrap(),
        element_splitted[1].parse::<i32>().unwrap(),
    )
}

fn solution_1(first: &str, second: &str) -> bool {
    let first_range = get_range_from_element(first);
    let second_range = get_range_from_element(second);
    first_range.contains_range(&second_range) || second_range.contains_range(&first_range)
}

fn solution_2(first: &str, second: &str) -> bool {
    let first_range = get_range_from_element(first);
    let second_range = get_range_from_element(second);
    first_range.has_common_elements_with(&second_range) || second_range.has_common_elements_with(&first_range)
}

impl Day for Day4 {
    fn solve(&self) -> String {
        let input = self.get_input("input4");
        let lines = input.lines();

        let mut n = 0;

        for line in lines {
            let elements: Vec<&str> = line.split(",").collect();
            n = n + solution_2(elements[0], elements[1]) as i32;
        }

        n.to_string()
    }
}
