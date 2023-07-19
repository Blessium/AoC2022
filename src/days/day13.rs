use crate::resolver::Day;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Day13;

impl Day13 {
    pub fn new() -> Self {
        Day13 {}
    }
}

pub struct Packet {
    values: Vec<i32>,
    next: Vec<Rc<RefCell<Packet>>>,
    prev: Option<Rc<RefCell<Packet>>>,
}

pub enum NextItem {
    LeftPar,
    RightPar,
    Comma,
    Number(i32),
}

impl NextItem {
    pub fn parse_next(item: char) -> Self {
        match item {
            '[' => NextItem::LeftPar,
            ']' => NextItem::RightPar,
            '0'..='9' => NextItem::Number(item as i32 - '0' as i32),
            ',' => NextItem::Comma,
            _ => panic!("What the hell is next_item"),
        }
    }
}

impl Packet {
    pub fn parse(mut value: String) -> Rc<RefCell<Packet>> {
        let mut current = Rc::new(RefCell::new(Packet {
            values: Vec::new(),
            next: vec![],
            prev: None,
        }));

        let mut is_ten = 0;
        let mut buffer: i32 = 0;

        let head = Rc::clone(&current);

        for char in value.chars().into_iter() {
            match NextItem::parse_next(char) {
                NextItem::LeftPar => {
                    if is_ten == 1 {
                        current.borrow_mut().values.push(buffer);
                        is_ten = 0;
                    }

                    current.borrow_mut().values.push(-1);
                    let next = Rc::new(RefCell::new(Packet {
                        values: Vec::new(),
                        next: vec![],
                        prev: Some(Rc::clone(&current)),
                    }));
                    current.borrow_mut().next.push(Rc::clone(&next));
                    current = Rc::clone(&next);
                }
                NextItem::RightPar => {
                    if is_ten == 1 {
                        current.borrow_mut().values.push(buffer);
                        is_ten = 0;
                    }

                    current.borrow_mut().values.push(-2);
                    let prev = Rc::clone(&current.borrow_mut().prev.as_ref().unwrap());
                    current = prev;
                }
                NextItem::Comma => {
                    if is_ten == 1 {
                        is_ten = 0;
                        current.borrow_mut().values.push(buffer);
                    }

                    continue;
                }
                NextItem::Number(number) => {
                    if is_ten == 1 {
                        current.borrow_mut().values.push(10);
                        is_ten = 0;
                    } else {
                        buffer = number;
                        is_ten += 1;
                    }
                }
            }
        }

        head
    }

    pub fn reconstruct(&self) {
        let mut index = 0;
        for value in &self.values {
            if *value == -1 {
                print!("[");
                self.next[index].borrow_mut().reconstruct();
                print!("],");
                index += 1;
            } else {
                print!("{},", value);
            }
        }
    }

    pub fn solve(first: Rc<RefCell<Packet>>, second: Rc<RefCell<Packet>>) -> bool {
        let current_first = Rc::clone(&first);
        let current_second = Rc::clone(&second);

        let mut result: bool = true;

        loop {
            let first_item = current_first.borrow_mut().pop_item();
            let second_item = current_second.borrow_mut().pop_item();

            match (first_item, second_item) {
                (None)
                (Some(first_number), None) => return true,
                _ => panic!("nigga"),
            }
        }
    }


    pub fn pop_item(&mut self) -> Option<i32> {
        self.values.reverse();
        let value = self.values.pop();
        self.values.reverse();
        value
    }
}

pub fn solution_1(lines: std::str::Lines) -> String {
    for (index, line) in lines.enumerate() {
        if index + 1 % 3 == 0 {
            continue;
        }

        println!("current line: {}", line);
        let packet = Packet::parse(line.to_string());
        packet.borrow_mut().reconstruct();
        println!("");
    }
    "day13".to_string()
}

impl Day for Day13 {
    fn solve(&self) -> String {
        let input = self.get_input("input13");
        solution_1(input.lines()).to_string()
    }
}
