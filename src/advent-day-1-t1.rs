use std::fs;

fn main() {
    let mut number_of_increments = 0;
    let mut current_number_of_increments = 0;
    let mut last_number_accessed: Option<i32> = None;


    let data = fs::read_to_string("./dummy_data.txt").expect("Unable to read file");
    for item in data.split('\n'){

        let parsed = item.parse::<i32>().unwrap();
        match last_number_accessed {
            Some(_) => (),
            None => last_number_accessed = Some(parsed)
        }

        
        if last_number_accessed.unwrap() < parsed {
            current_number_of_increments += 1;
        } else {
            number_of_increments += current_number_of_increments;
            current_number_of_increments = 0;
        }
        last_number_accessed = Some(parsed);
    }
    number_of_increments += current_number_of_increments;
    println!("{}", number_of_increments);

}