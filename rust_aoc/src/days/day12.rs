use crate::resolver::Day;
use core::cmp::Reverse;
use std::collections::binary_heap::BinaryHeap;
use std::collections::HashSet;

pub struct Day12;

impl Day12 {
    pub fn new() -> Self {
        Day12 {}
    }
}
#[derive(PartialOrd, Ord, PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Position {
    x: isize,
    y: isize,
}

impl Position {
    pub fn new(y: isize, x: isize) -> Self {
        Position { x, y }
    }

    pub fn add(&self, another: Position) -> Position {
        Position {
            x: self.x + another.x,
            y: self.y + another.y,
        }
    }
}

#[derive(Ord, PartialOrd, PartialEq, Eq, Hash, Clone, Debug, Copy)]
pub struct Node {
    cost: isize,
    position: Position,
}

pub struct Graph {
    values: Vec<Vec<usize>>,
    target: Position,
}

impl Graph {
    pub fn new(graph: Vec<Vec<usize>>, target: Position) -> Self {
        Graph {
            values: graph,
            target,
        }
    }

    fn is_valid(&self, pos: Position) -> bool {
        if pos.y < 0
            || pos.x < 0
            || pos.x >= self.values[0].len().try_into().unwrap()
            || pos.y >= self.values.len().try_into().unwrap()
        {
            false
        } else {
            true
        }
    }

    pub fn solution1(&mut self, starting: Position) -> Option<usize> {
        let adj_location: Vec<Position> = vec![
            Position::new(1, 0),
            Position::new(0, 1),
            Position::new(-1, 0),
            Position::new(0, -1),
        ];

        let mut visited: HashSet<Position> = HashSet::new();
        let mut pq: BinaryHeap<Reverse<Node>> = BinaryHeap::new();

        pq.push(Reverse(Node {
            cost: 0,
            position: starting,
        }));

        while let Some(node) = pq.pop() {
            if node.0.position == self.target {
                return Some(node.0.cost as usize);
            }

            let current_value = self.values[node.0.position.y as usize][node.0.position.x as usize];

            visited.insert(node.0.position);

            for adj in adj_location.iter() {
                let adj_position = node.0.position.clone().add(*adj);
                if self.is_valid(adj_position) {
                    let adj_value = self.values[adj_position.y as usize][adj_position.x as usize];
                    if current_value == ('S' as usize)
                        || (adj_value <= current_value)
                        || adj_value == (current_value + 1)
                    {
                        if visited.insert(adj_position) {
                            pq.push(Reverse(Node {
                                cost: (node.0.cost + 1) as isize,
                                position: adj_position,
                            }));
                        }
                    }
                }
            }
        }
        None
    }
}

pub fn solution1(lines: std::str::Lines) {
    let mut values: Vec<Vec<usize>> = Vec::new();
    let mut starting_point: Position = Position::new(0, 0);
    let mut target: Position = Position::new(0, 0);
    for (index, line) in lines.enumerate() {
        values.push(
            line.chars()
                .into_iter()
                .enumerate()
                .inspect(|(j, x)| {
                    if *x == 'S' {
                        starting_point = Position::new(index as isize, *j as isize);
                    } else if *x == 'E' {
                        target = Position::new(index as isize, *j as isize);
                    }
                })
                .map(|(_, x)| if x == 'E' { '{' as usize } else { x as usize })
                .collect::<Vec<usize>>(),
        );
    }

    let mut graph = Graph::new(values, target);
    if let Some(result) = graph.solution1(starting_point) {
        println!("{}", result);
    }
}

pub fn solution2(lines: std::str::Lines) {
    let mut values: Vec<Vec<usize>> = Vec::new();
    let mut starting_point: Position = Position::new(0, 0);
    let mut target: Position = Position::new(0, 0);
    let mut targets: Vec<Position> = Vec::new();
    for (index, line) in lines.enumerate() {
        values.push(
            line.chars()
                .into_iter()
                .enumerate()
                .inspect(|(j, x)| {
                    if *x == 'S' {
                        starting_point = Position::new(index as isize, *j as isize);
                    } else if *x == 'E' {
                        target = Position::new(index as isize, *j as isize);
                    } else if *x == 'a' {
                        targets.push(Position::new(index as isize, *j as isize));
                    }
                })
                .map(|(_, x)| if x == 'E' { '{' as usize } else { x as usize })
                .collect::<Vec<usize>>(),
        );
    }

    let mut graph = Graph::new(values, target);
    let mut values: Vec<usize> = Vec::new();
    for aaes in targets {
        if let Some(result) = graph.solution1(aaes) {
            values.push(result);
        }
    }

    values.sort();
    values.reverse();
    println!("Result: {}", values.pop().unwrap());
}

impl Day for Day12 {
    fn solve(&self) -> String {
        let input = self.get_input("input12");
        solution2(input.lines());
        "day12".to_string()
    }
}
