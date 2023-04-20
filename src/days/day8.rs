use crate::resolver::Day;

pub struct Day8;

impl Day8 {
    pub fn new() -> Self {
        Day8 {}
    }
}

pub fn build_grid(lines: std::str::Lines) -> Vec<Vec<u8>> {
    let mut temp: Vec<Vec<u8>> = Vec::new();
    for line in lines {
        temp.push(
            line.chars()
                .into_iter()
                .map(|x| x as u8 - '0' as u8)
                .collect::<Vec<u8>>(),
        );
    }
    temp
}

// from day 5
fn matrix_transpose(m: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let mut t = vec![Vec::with_capacity(m.len()); m[0].len()];
    for r in m {
        for i in 0..r.len() {
            t[i].push(r[i]);
        }
    }
    t
}

pub fn check_if(vec: &Vec<u8>, number: u8) -> bool {
    let mut cloned = vec.clone();
    cloned.sort();
    let number2 = cloned.pop().unwrap();
    if number > number2 {
        true
    } else {
        false
    }
}

pub fn get_if(vec: &Vec<u8>, number: u8) -> i32 {
    let mut index = 0;
    for n in vec {
        index += 1;
        if number <= *n {
            return index;
        }
    }

    index
}

pub fn solution_1(lines: std::str::Lines) -> String {
    let mut vertical_grid: Vec<Vec<u8>> = build_grid(lines);
    let mut horizontal_grid: Vec<Vec<u8>> = matrix_transpose(vertical_grid.clone());

    let len = vertical_grid.len();

    let mut count = len * 4 - 4;

    for i in 1..len - 1 {
        for j in 1..len - 1 {
            let mut right = vertical_grid[i].split_off(j);
            let mut right_hor = horizontal_grid[j].split_off(i);

            let number = right.remove(0);
            let _ = right_hor.remove(0);

            if check_if(&right, number)
                || check_if(&right_hor, number)
                || check_if(&vertical_grid[i], number)
                || check_if(&horizontal_grid[j], number)
            {
                count += 1;
            }

            right.insert(0, number);
            right_hor.insert(0, number);

            vertical_grid[i].extend(right);
            horizontal_grid[j].extend(right_hor);
        }
    }

    count.to_string()
}

pub fn solution_2(lines: std::str::Lines) -> String {
    let mut vertical_grid: Vec<Vec<u8>> = build_grid(lines);
    let mut horizontal_grid: Vec<Vec<u8>> = matrix_transpose(vertical_grid.clone());

    let len = vertical_grid.len();

    let mut current = -1;

    for i in 1..len - 1 {
        for j in 1..len - 1 {
            let mut right = vertical_grid[i].split_off(j);
            let mut right_hor = horizontal_grid[j].split_off(i);

            let number = right.remove(0);
            let _ = right_hor.remove(0);

            let mut right_another = vertical_grid[i].clone();
            right_another.reverse();
            let mut right_h_ano = horizontal_grid[j].clone();
            right_h_ano.reverse();

            let result = get_if(&right, number)
                * get_if(&right_hor, number)
                * get_if(&right_another, number)
                * get_if(&right_h_ano, number);

            if result >= current {
                current = result
            }

            right.insert(0, number);
            right_hor.insert(0, number);

            vertical_grid[i].extend(right);
            horizontal_grid[j].extend(right_hor);
        }
    }

    current.to_string()
}

impl Day for Day8 {
    fn solve(&self) -> String {
        let input = self.get_input("input8");
        solution_2(input.lines())
    }
}
