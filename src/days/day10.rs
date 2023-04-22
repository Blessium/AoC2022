use crate::resolver::Day;

pub struct Day10;

impl Day10 {
    pub fn new() -> Self {
        Day10 {}
    }
}

pub enum Instruction {
    Noop,
    Addx(isize),
}

impl Instruction {
    pub fn new(parse: String) -> Self {
        let mut instruction = parse.split_ascii_whitespace();
        match instruction.next() {
            Some("noop") => Instruction::Noop,
            Some("addx") => Instruction::Addx(instruction.next().unwrap().parse().unwrap()),
            _ => panic!("not a valid instruction"),
        }
    }
}

pub struct Computer {
    strength: isize,
    cycle: isize,
    x: isize,
    sprite: Vec<char>,
    crt: Vec<Vec<char>>,
}

impl Computer {
    pub fn new() -> Self {
        let mut sprite: Vec<char> = Vec::new();
        let temp = vec!['#'; 3];
        let temp2 = vec!['.'; 37];
        sprite.extend(temp);
        sprite.extend(temp2);
        Computer {
            strength: 0,
            cycle: 0,
            x: 1,
            sprite,
            crt: vec![vec!['.'; 40]; 6],
        }
    }

    pub fn instruction(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Noop => {
                self.set_crt();
                self.cycle += 1;
                self.check_strength();
            }

            Instruction::Addx(size) => {
                for _ in 0..2 {
                    self.set_crt();
                    self.cycle += 1;
                    self.check_strength();
                }
                self.x += size;
            }
        }
        self.set_sprite_position();
    }

    pub fn check_strength(&mut self) {
        let signal: Vec<isize> = vec![20, 60, 100, 140, 180, 220];

        if signal.contains(&self.cycle) {
            self.strength += self.cycle * self.x;
            println!(
                "Current cycle {} x {} strength {}",
                self.cycle,
                self.x,
                self.cycle * self.x
            );
        }
    }

    pub fn get_strength(&self) -> isize {
        self.strength
    }

    pub fn set_sprite_position(&mut self) {
        self.sprite = self
            .sprite
            .iter()
            .enumerate()
            .map(|(index, _)| {
                let frame = (self.x - 1)..=(self.x + 1);
                if frame.contains(&(index as isize)) {
                    '#'
                } else {
                    '.'
                }
            })
            .collect();
        self.print_sprite();
    }

    pub fn set_crt(&mut self) {
        let range = (self.x - 1)..=(self.x + 1);
        let curr_row = self.cycle as usize / self.crt[0].len();
        let curr_column = self.cycle as usize % self.crt[0].len();
        println!("crt[{}][{}]", curr_row, curr_column);

        if range.contains(&(curr_column as isize)) {
            self.crt[curr_row][curr_column] = '#';
        }
        self.print_crt();
    }

    pub fn print_crt(&self) {
        println!("");
        for i in 0..self.crt.len() {
            for j in 0..self.crt[i].len() {
                print!("{}", self.crt[i][j]);
            }
            println!("");
        }
    }

    pub fn print_sprite(&self) {
        println!("");
        println!("x: {}", self.x);
        for char in self.sprite.iter() {
            print!("{}", char);
        }
        println!("");
    }
}

pub fn solution_1(lines: std::str::Lines) -> String {
    let mut cpu = Computer::new();

    for line in lines {
        let instruction = Instruction::new(line.to_string());
        cpu.instruction(instruction);
    }

    cpu.get_strength().to_string()
}

impl Day for Day10 {
    fn solve(&self) -> String {
        let input = self.get_input("input10");

        solution_1(input.lines())
    }
}
