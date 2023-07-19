use crate::resolver::Day;

pub struct Day1 {}

impl Day1 {
    pub fn new() -> Self {
        Day1 {}
    }
}

impl Day for Day1 {
    fn solve(&self) -> String {
        let input = self.get_input("input1");
        let lines: Vec<&str> = input.split("\n").collect();
        let mut sums: Vec<i64> = Vec::new();
        let mut sum = 0;

        for number in lines {
            if let Ok(value) = number.parse::<i64>() {
                sum += value;
            } else {
                sums.push(sum);
                sum = 0;
            }
        }

        // solution number 1
        let _sol1 = sums.iter().max().unwrap();
        sums.sort();
        sums.reverse();
        let sol2: i64 = sums[..3].to_vec().iter().sum();
        sol2.to_string()
    }
}
