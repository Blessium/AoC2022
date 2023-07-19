use crate::resolver::Day;

pub struct Day6;

impl Day6 {
    pub fn new() -> Self {
        Day6 {}
    }
}

pub fn solution_12(input: String, cons: usize) -> usize {
    let mut buffer: Vec<char> = Vec::new();
    let mut already_found = false;
    let mut result = 0;
    let _ = input
        .chars()
        .into_iter()
        .enumerate()
        .map(|(index, value)| {
            buffer.push(value);
            if (buffer.len() == cons) && !already_found {
                let mut tmp = buffer.clone();
                tmp.sort();
                tmp.dedup();
                if tmp.len() == cons {
                    already_found = true;
                    result = index + 1;
                }
                buffer = buffer[1..].to_vec();
            }
            value
        })
        .collect::<Vec<char>>();

    result
}

impl Day for Day6 {
    fn solve(&self) -> String {
        let input = self.get_input("input6");
        solution_12(input.trim().to_string(), 4).to_string()

        // this for solution2
        // solution_12(input.trim().to_string(), 14).to_string()
    }
}
