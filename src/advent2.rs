
use std::fs;

#[derive(Debug)]
struct Incr {
    first: Option<i32>,
    second: Option<i32>,
    third: Option<i32>,
    increment: Option<i32>,
    increased: i32,
}

impl Incr {
    fn new() -> Incr {
        Incr{first: None, second: None, third: None, increment: None, increased: 0 }
    }
    fn shift(&mut self, update_value: i32){
        self.first = self.second;
        self.second = self.third;
        self.third = Some(update_value);
        self.is_ready()
    }
    fn count_increment(&mut self) {
        let combined = self.first.unwrap() + self.second.unwrap() + self.third.unwrap();
        match &self.increment {
            None => { self.increment = Some(combined) },
            Some(x) if x < &combined => {self.increment = Some(combined); self.increased +=1 },
            _ => {self.increment = Some(combined) }        
        }
    }
    fn is_ready(&mut self) {
        match &self {
            Incr { first: Some(_), second: Some(_), third: Some(_), .. }  => self.count_increment(),
            _ => ()
        }
    }
}


fn main(){
    let mut obj = Incr::new();
    let mut last_number_accessed: Option<i32> = None;
    let data = fs::read_to_string("./dummy_data.txt").expect("Unable to read file");
    for item in data.split('\n'){

        let parsed = item.parse::<i32>().unwrap();
        match last_number_accessed {
            Some(_) => (),
            None => last_number_accessed = Some(parsed)
        }

        obj.shift(parsed)

    }
    println!("{}", obj.increased)
}