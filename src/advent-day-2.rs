use std::fs;
use std::str::FromStr;


#[derive(Debug)]
struct Position {
    x: i32,
    y: i32
}

enum Movement {
    Forward,
    Down,
    Up,
    Backward
}

impl Position {
    fn new() -> Position {
        Position { x: 0, y: 0}
    }
    fn mv(&mut self, direction: Movement, distance: i32){
        match direction {
            Movement::Forward => {self.x += distance},
            Movement::Backward => {self.x -= distance},
            Movement::Down => {self.y += distance},
            Movement::Up => {self.y -= distance},
        };
    }
}

impl FromStr for Movement {

    type Err = ();

    fn from_str(input: &str) -> Result<Movement, Self::Err> {
        match input {
            "forward"  => Ok(Movement::Forward),
            "backward"  => Ok(Movement::Backward),
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
            Some((distance, direction)) => { obj.mv(Movement::from_str(distance).unwrap(), direction.parse::<i32>().unwrap())},
            _ => ()
        }
    }
    let result = obj.x * obj.y;
    println!("{:#?}", result)
}