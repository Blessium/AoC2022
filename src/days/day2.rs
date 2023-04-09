
use crate::resolver::Day;

pub struct Day2 {}

impl Day2 {
    pub fn new() -> Self {
        Day2 {}
    }
}

fn shape_select(shape: char) -> i64{
    match shape {
        'X'|'A' => 1, // Rock
        'Y'|'B' => 2, // Paper
        'Z'|'C' => 3, // Scissors
        _ => 0
    }
}

fn get_result(me: char, op: char) -> i64 {
    let val = shape_select(me);
    let op_val = shape_select(op);

    if op_val == val {
        println!("draw");
        3 + val
    } else if (val == 2 && op_val == 1) || (val == 1 && op_val == 3) || (val == 3 && op_val == 2) {
        println!("win");
        6 + val 
    } else {
        println!("lost");
        0 + val
    }
}

impl Day for Day2 {
    fn solve(&self) -> String {
        let input =  self.get_input("input2");
        let mut play: Vec<&str> = input.split("\n").collect();
        println!("playlen: {}", play.len());
        play.remove(play.len()-1);

        let mut total_score: i64 = 0;

        for game in play {
           let shapes: Vec<char> = game.split(" ")
                                    .collect::<Vec<&str>>()
                                    .iter()
                                    .filter(|word| !word.is_empty())
                                    .map(| c | c.chars().next().unwrap()).collect::<Vec<char>>();     

           println!("me {} enemy {}", shapes[1], shapes[0]);
           total_score += get_result(shapes[1], shapes[0]);
        }

        total_score.to_string()
    }
}
