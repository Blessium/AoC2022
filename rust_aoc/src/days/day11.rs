use crate::resolver::Day;
use std::collections::binary_heap::BinaryHeap;

pub struct Day11;

impl Day11 {
    pub fn new() -> Self {
        Day11 {}
    }
}

#[derive(Debug, Clone)]
pub struct Monkey {
    items: Vec<usize>,
    count: usize,
    operation: Operation,
    test_div: usize,
    throw_monkeys: Vec<usize>,
}

#[derive(Debug, Clone, Copy)]
pub enum Operation {
    Sum(usize),
    Mult(usize),
    OldSum,
    OldMut,
}

pub struct Inspection(usize, usize);

impl Monkey {
    pub fn parse(mut str: Vec<&str>) -> Monkey {
        str.pop().unwrap();
        let items = Monkey::parse_items(str.pop().unwrap());
        let operation = Monkey::parse_op(str.pop().unwrap());
        let test_div = Monkey::parse_div(str.pop().unwrap());
        let throw_monkeys = Monkey::parse_condition(str.pop().unwrap(), str.pop().unwrap());
        Monkey {
            items,
            count: 0,
            operation,
            test_div,
            throw_monkeys,
        }
    }

    fn parse_items(str: &str) -> Vec<usize> {
        let numbers = str.split(":").collect::<Vec<&str>>().pop().unwrap();
        numbers
            .split(",")
            .into_iter()
            .map(|str| str.trim().parse::<usize>().unwrap())
            .collect::<Vec<usize>>()
    }

    fn parse_op(str: &str) -> Operation {
        let op = str.split("=").collect::<Vec<&str>>().pop().unwrap();

        let op_normale = vec![Operation::Sum, Operation::Mult];
        let op_old = vec![Operation::OldSum, Operation::OldMut];

        for (i, operation) in vec!["+", "*"].into_iter().enumerate() {
            if op.contains(operation) {
                let x = op.split(operation).collect::<Vec<&str>>().pop().unwrap();
                if let Ok(number) = x.trim().parse::<usize>() {
                    return op_normale[i](number);
                } else {
                    return op_old[i];
                }
            }
        }
        panic!("not valid");
    }

    fn parse_div(str: &str) -> usize {
        str.split_ascii_whitespace()
            .collect::<Vec<&str>>()
            .pop()
            .unwrap()
            .parse()
            .unwrap()
    }
    fn parse_condition(tru: &str, fals: &str) -> Vec<usize> {
        let mut r: Vec<usize> = Vec::new();
        r.push(
            fals.split_ascii_whitespace()
                .collect::<Vec<&str>>()
                .pop()
                .unwrap()
                .parse()
                .unwrap(),
        );
        r.push(
            tru.split_ascii_whitespace()
                .collect::<Vec<&str>>()
                .pop()
                .unwrap()
                .parse()
                .unwrap(),
        );
        r
    }

    pub fn add_item(&mut self, item: usize) {
        self.items.push(item);
    }

    pub fn get_count(&self) -> usize {
        self.count
    }

    fn increase_worry(&mut self, item: usize) -> usize {
        match self.operation {
            Operation::OldMut => item * item,
            Operation::OldSum => item + item,
            Operation::Mult(n) => item * n,
            Operation::Sum(n) => item + n,
        }
    }

    pub fn get_div(&self) -> usize {
        self.test_div
    }

    pub fn inspect_item(&mut self, com: Option<usize>) -> Option<Inspection> {
        self.items.reverse();
        if let Some(item) = self.items.pop() {
            self.count += 1;
            let mut item = (self.increase_worry(item)) as usize;
            if let Some(n) = com {
                item = item % n;
            }
            self.items.reverse();
            let location = item % self.test_div == 0;
            Some(Inspection(item, self.throw_monkeys[location as usize]))
        } else {
            self.items.reverse();
            None
        }
    }
}

pub struct Game {
    monkeys: Vec<Monkey>,
}

impl Game {
    pub fn new() -> Self {
        Game {
            monkeys: Vec::new(),
        }
    }

    pub fn init(&mut self, lines: std::str::Lines) {
        let mut monkeys_input: Vec<&str> = vec![];

        for (i, line) in lines.enumerate() {
            if (i + 1) % 7 == 0 {
                let monkey = Monkey::parse(monkeys_input.clone());
                self.monkeys.push(monkey);
                monkeys_input.clear();
                continue;
            }
            monkeys_input.insert(0, line.trim());
        }
    }

    pub fn round(&mut self) {
        for i in 0..self.monkeys.len() {
            while let Some(inspection) = self.monkeys[i].inspect_item(None) {
                self.monkeys[inspection.1].add_item(inspection.0);
            }
        }
    }
    pub fn round_2(&mut self) {
        let commond_div = self.common_multiple();
        for i in 0..self.monkeys.len() {
            while let Some(inspection) = self.monkeys[i].inspect_item(Some(commond_div)) {
                self.monkeys[inspection.1].add_item(inspection.0);
            }
        }
    }

    pub fn debug(&self) {
        for monkey in self.monkeys.iter() {
            println!("{:?}", monkey);
        }
    }

    pub fn get_monkey_business(&self) -> usize {
        let mut priority = BinaryHeap::new();
        for monkey in self.monkeys.iter() {
            priority.push(monkey.get_count());
        }

        let mut result = priority.pop().unwrap();
        result *= priority.pop().unwrap();
        result
    }

    pub fn common_multiple(&self) -> usize {
        self.monkeys.iter().map(|x| x.get_div()).product()
    }
}

pub fn solution1(lines: std::str::Lines) -> String {
    let mut game = Game::new();
    game.init(lines);
    for i in 0..20 {
        game.round();
    }
    game.get_monkey_business().to_string()
}

pub fn solution2(lines: std::str::Lines) -> String {
    let mut game = Game::new();
    game.init(lines);
    for i in 0..10000 {
        game.round_2();
    }
    game.get_monkey_business().to_string()
}

impl Day for Day11 {
    fn solve(&self) -> String {
        let input = self.get_input("input11");
        solution2(input.lines())
    }
}
