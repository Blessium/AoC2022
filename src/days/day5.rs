use crate::resolver::Day;

pub struct Day5;

impl Day5 {
    pub fn new() -> Self {
        Day5 {}
    }
}

struct Command {
    pub n: usize,
    pub from: usize,
    pub to: usize,
}

pub fn clean_str(string: String) -> String {
    string.trim().replace("[", "").replace("]", "")
}

pub fn split_every_n(string: &str, n: usize) -> Vec<String> {
    let mut solution: Vec<String> = Vec::new();
    string
        .chars()
        .into_iter()
        .enumerate()
        .fold(Vec::<char>::new(), |mut result, (index, char)| {
            if (index + 1) % n != 0 && (index + 1 != string.len()) {
                result.push(char);
            } else {
                solution.push(clean_str(result.clone().into_iter().collect::<String>()));
                result.clear();
            }
            result
        });
    solution
}

fn matrix_transpose_and_reverse(m: Vec<Vec<String>>) -> Vec<Vec<String>> {
    let mut t = vec![Vec::with_capacity(m.len()); m[0].len()];
    for r in m {
        for i in 0..r.len() {
            if r[i].len() != 0 {
                t[i].insert(0, r[i].clone());
            }
        }
    }
    t
}

fn create_table(mut table: Vec<&str>) -> Vec<Vec<String>> {
    table.pop().unwrap();

    let mut result: Vec<Vec<String>> = Vec::<Vec<String>>::new();

    for row in table.into_iter() {
        result.push(split_every_n(row, 4));
    }

    matrix_transpose_and_reverse(result).to_owned()
}

fn get_command(line: &str) -> Command {
    let numbers: Vec<usize> = line
        .split_whitespace()
        .filter(|x| x.parse::<usize>().is_ok())
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    Command {
        n: numbers[0],
        from: numbers[1] - 1,
        to: numbers[2] - 1,
    }
}

pub fn solution_facade<P>(lines: std::str::Lines, mut predicate: P) -> String
where
    P: FnMut(Vec<String>) -> Vec<String>
{
    let mut index: usize = 0;
    let table_values: Vec<&str> = lines
        .clone()
        .into_iter()
        .enumerate()
        .take_while(|(i, x)| {
            index = *i;
            x.len() != 0
        })
        .map(|(_, x)| x)
        .collect::<Vec<&str>>();

    let mut table = create_table(table_values);

    for line in lines.skip(index + 1) {
        let command: Command = get_command(line);
        let truck_index = table[command.from].len() - command.n;
        let mut truncated_part = table[command.from].split_off(truck_index);
        truncated_part = predicate(truncated_part);
        table[command.to].extend(truncated_part);
    }

    let mut result: String = String::new();
    for mut row in table {
        result.push(row.pop().unwrap().chars().next().unwrap());
    }
    result
}

pub fn solution_2(tmp: Vec<String>) -> Vec<String> {
    tmp
}

pub fn solution_1(mut tmp: Vec<String>) -> Vec<String> {
    tmp.reverse();
    tmp
}

impl Day for Day5 {
    fn solve(&self) -> String {
        let input = self.get_input("input5");
        solution_facade(input.lines(), solution_2)
    }
}
