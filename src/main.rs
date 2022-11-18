use std::fs;
use std::str::FromStr;


#[derive(Debug)]
struct Position {
    horizontal_distance: u32,
    depth: u32,
    aim: u32
}

enum Movement {
    Forward,
    Down,
    Up
}

impl Position {
    fn new() -> Position {
        Position { horizontal_distance: 0, depth: 0, aim: 0}
    }
    fn mv(&mut self, direction: Movement, distance: u32){
        match direction {
            Movement::Forward => {self.horizontal_distance += distance; self.depth += distance * self.aim},
            Movement::Down => {self.aim += distance},
            Movement::Up => {self.aim -= distance},
        };
    }
}

impl FromStr for Movement {

    type Err = ();

    fn from_str(input: &str) -> Result<Movement, Self::Err> {
        match input {
            "forward"  => Ok(Movement::Forward),
            "up"  => Ok(Movement::Up),
            "down" => Ok(Movement::Down),
            _      => Err(()),
        }
    }
}

fn main (){
    let mut obj = Position::new();
    let data = fs::read_to_string("./dummy_data.txt").expect("Unable to read file");
    for item in data.split('\n'){
        match item.split_once(' '){
            Some((distance, direction)) => { obj.mv(Movement::from_str(distance).unwrap(), direction.parse::<u32>().unwrap())},
            _ => ()
        }
    }
    let result = obj.horizontal_distance * obj.depth;
    println!("{:#?}", result)
}