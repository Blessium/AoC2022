use crate::resolver::Day;
use std::collections::HashSet;

pub struct Day9;

impl Day9 {
    pub fn new() -> Self {
        Day9 {}
    }
}

#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
struct Position {
    pub x: i64,
    pub y: i64,
}

impl Position {
    pub fn get_diff(&self, other: &Position) -> Position {
        Position { x: (self.x - other.x), y: (self.y - other.y)}
    }
}

struct RopeSimulation {
    known_pos: HashSet<Position>,
    curr_tail: Position,
    curr_head: Position,
    snake: Vec<Position>,
}

impl RopeSimulation {
    pub fn new() -> Self {
        RopeSimulation {
            known_pos: HashSet::new(),
            curr_tail: Position { x: 0, y: 0 },
            curr_head: Position { x: 0, y: 0 },
            snake: vec![Position { x: 0, y: 0}; 10],
        }
    }

    pub fn r#move(&mut self, dir: &str, n: isize) {
        for i in 0..n {
            match dir {
                "U" => self.curr_head.y -= 1,
                "D" => self.curr_head.y += 1,
                "L" => self.curr_head.x -= 1,
                "R" => self.curr_head.x += 1,
                _ => panic!("unknown direction"),
            }

            let xdiff = self.curr_head.x - self.curr_tail.x;
            let ydiff = self.curr_head.y - self.curr_tail.y;

            if xdiff == 0 && ydiff.abs() > 1 {
                self.curr_tail.y += ydiff.signum();
            } else if ydiff == 0 && xdiff.abs() > 1 {
                self.curr_tail.x += xdiff.signum();
            } else if xdiff.abs() > 1 || ydiff.abs() > 1 {
                self.curr_tail.x += xdiff.signum(); 
                self.curr_tail.y += ydiff.signum();
            }

            self.known_pos.insert(self.curr_tail);
        }
    }

    pub fn move2(&mut self, dir: &str, n: isize) {
        for _ in 0..n {
            match dir {
                "U" => self.snake[0].y -= 1,
                "D" => self.snake[0].y += 1,
                "L" => self.snake[0].x -= 1,
                "R" => self.snake[0].x += 1,
                _ => panic!("unknown direction"),
            }

            for i in 1..10 {
                let xdiff = self.snake[i - 1].x - self.snake[i].x;
                let ydiff = self.snake[i-1].y - self.snake[i].y;

                if xdiff == 0 && ydiff.abs() > 1 {
                    self.snake[i].y += ydiff.signum();
                } else if ydiff == 0 && xdiff.abs() > 1 {
                    self.snake[i].x += xdiff.signum();
                } else if xdiff.abs() > 1 || ydiff.abs() > 1 {
                    self.snake[i].x += xdiff.signum(); 
                    self.snake[i].y += ydiff.signum();
                }

            }
            self.known_pos.insert(self.snake[9]);
        }
    }

    pub fn get_result(&self) -> usize {
        self.known_pos.len()
    }

}

pub fn solution_1(lines: std::str::Lines) -> String {
    let mut rope_simulation = RopeSimulation::new();
    for line in lines {
        let mut s = line.split_ascii_whitespace();
        let direction = s.next().unwrap();
        let number = s.next().unwrap().parse::<isize>().unwrap();
        rope_simulation.r#move2(direction, number);
    }
    rope_simulation.get_result().to_string()
}

pub fn solution_2(lines: std::str::Lines) -> String {
    let mut rope_simulation = RopeSimulation::new();
    for line in lines {
        let mut s = line.split_ascii_whitespace();
        let direction = s.next().unwrap();
        let number = s.next().unwrap().parse::<isize>().unwrap();
        rope_simulation.move2(direction, number);
    }
    rope_simulation.get_result().to_string()
}
impl Day for Day9 {
    fn solve(&self) -> String {
        let input = self.get_input("input9");
        let lines = input.lines();
        solution_2(lines).to_string()
    }
}
