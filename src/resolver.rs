use std::fs;

pub struct Resolver<T> {
    day: T,
}

impl<T> Resolver<T>
where
    T: Day,
{
    pub fn new(day: T) -> Self {
        Resolver { day }
    }

    pub fn get_output(&self) {
        println!("Result");
        println!("{}", self.day.solve());
    }
}

pub trait Day {
    fn get_input(&self, file_name: &str) -> String {
        let mut input_file = String::from("inputs/");
        input_file.push_str(file_name);
        fs::read_to_string(input_file).expect("No correct input file")
    }
    fn solve(&self) -> String;
}
