use crate::resolver::Day;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Day7;

impl Day7 {
    pub fn new() -> Self {
        Day7 {}
    }
}

#[derive(Debug)]
struct DirectoryTree {
    name: String,
    dirsize: usize,
    visited: bool,
    subdir: Vec<Rc<RefCell<DirectoryTree>>>,
    parent: Option<Rc<RefCell<DirectoryTree>>>,
}

impl DirectoryTree {
    pub fn new(
        name: Option<String>,
        parent: Option<Rc<RefCell<DirectoryTree>>>,
    ) -> Rc<RefCell<DirectoryTree>> {
        Rc::new(RefCell::new(DirectoryTree {
            name: name.unwrap_or(String::from("/")),
            dirsize: 0,
            visited: false,
            subdir: vec![],
            parent,
        }))
    }

    pub fn is_visited(&mut self) -> bool {
        self.visited
    }

    pub fn set_visit(&mut self, visited: bool) {
        self.visited = visited;
    }

    pub fn add_directory(&mut self, dir: Rc<RefCell<DirectoryTree>>) {
        self.subdir.push(dir);
    }

    pub fn cd_directory(&mut self, dir_name: String) -> Option<Rc<RefCell<DirectoryTree>>> {
        if dir_name == ".." {
            if let Some(parent) = &self.parent {
                return Some(Rc::clone(parent));
            } else {
                return None;
            }
        }

        for dir in self.subdir.clone().into_iter() {
            if dir.borrow_mut().name == dir_name {
                return Some(Rc::clone(&dir));
            }
        }
        None
    }

    pub fn add_file_size(&mut self, size: usize) {
        self.dirsize += size;
    }

    fn go_to_root(dir: Rc<RefCell<DirectoryTree>>) -> Rc<RefCell<DirectoryTree>> {
        let mut current_dir = dir;
        loop {
            let temp: Rc<RefCell<DirectoryTree>>;
            if let Some(prev_dir) = &current_dir.borrow_mut().parent {
                temp = Rc::clone(&prev_dir);
            } else {
                break;
            }
            current_dir = temp;
        }
        current_dir
    }

    pub fn add_indirect_size(dir_start: Rc<RefCell<DirectoryTree>>) {
        let indirect_size = dir_start.borrow_mut().dirsize;
        let mut current_dir = dir_start;

        loop {
            let temp: Rc<RefCell<DirectoryTree>>;
            if let Some(prev_dir) = &current_dir.borrow_mut().parent {
                temp = Rc::clone(&prev_dir);
                temp.borrow_mut().dirsize += indirect_size;
            } else {
                break;
            }
            current_dir = temp;
        }

    }

    fn get_solution_1(&mut self, depth: &mut usize) -> usize {
        let mut sum = 0;
        let mut new_depth = *depth + 1;

        for subdir in self.subdir.clone().into_iter() {
                sum += subdir.borrow_mut().get_solution_1(&mut new_depth);
        }

        let result = self.dirsize + sum;

        if self.dirsize > 100000 {
            return sum; 
        } else {
            return result;
        }
    }
}

enum ElementType {
    Dir(String),
    File(usize),
}

impl ElementType {
    pub fn parse(element: String) -> Option<Self> {
        let mut element = element.split_ascii_whitespace();
        match element.next() {
            Some("dir") => Some(ElementType::Dir(element.next().unwrap().to_string())),
            Some(size) => Some(ElementType::File(size.parse::<usize>().unwrap())),
            None => panic!("what the fuck"),
        }
    }
}

enum Command {
    LS,
    CD(String),
}

impl Command {
    pub fn parse(element: String) -> Self {
        let mut element = element.split_ascii_whitespace();
        match element.next() {
            Some("ls") => Command::LS,
            Some("cd") => Command::CD(element.next().unwrap().to_string()),
            Some(_) | None => panic!("what the fuck"),
        }
    }
}

fn solution_1(lines: std::str::Lines) -> String {
    let mut dir_tree = DirectoryTree::new(None, None);
    let mut last_command = Command::CD(String::from("/"));
    for line in lines.skip(1) {
        if line.starts_with("$") {
            match &last_command {
                Command::LS => {
                    dir_tree.borrow_mut().set_visit(true);
                    DirectoryTree::add_indirect_size(Rc::clone(&dir_tree));
                }
                _ => (),
            }

            let (_, command) = line.split_at(1);
            match Command::parse(command.trim().to_string()) {
                Command::CD(dir) => {
                    let check_dir = dir_tree.borrow_mut().cd_directory(dir);
                    match check_dir {
                        Some(new_dir) => dir_tree = new_dir,
                        None => panic!("what the fuck new_dir"),
                    }
                    last_command = Command::CD("asdj".to_string());
                    continue;
                }
                Command::LS => {
                    last_command = Command::LS;
                    continue;
                }
            }
        }

        if dir_tree.borrow_mut().is_visited() {
            continue;
        }

        match ElementType::parse(line.to_string()) {
            Some(ElementType::Dir(element)) => {
                let parent = Rc::clone(&dir_tree);
                dir_tree
                    .borrow_mut()
                    .add_directory(DirectoryTree::new(Some(element), Some(parent)));
            }
            Some(ElementType::File(size)) => dir_tree.borrow_mut().add_file_size(size),
            None => panic!("what the fuck dir_tree element_type"),
        }
    }
    let x = DirectoryTree::go_to_root(dir_tree);
    let mut x = x.borrow_mut();
    let mut n: usize = 0;
    x.get_solution_1(&mut n).to_string()
}

impl Day for Day7 {
    fn solve(&self) -> String {
        let input = self.get_input("input7");
        let lines = input.lines();
        solution_1(lines)
    }
}
