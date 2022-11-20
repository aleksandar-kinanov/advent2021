use std::fs;
fn extract(mut input: Vec<&str>, switch: bool) -> &str {
    let mut _ones: Vec<&str> = Vec::new();
    let mut _zeroes: Vec<&str> = Vec::new();
    let mut counter = 0;
    let break_point = input.clone().iter().nth(0).unwrap().len();

    while counter < break_point {
        (_zeroes, _ones) = input
            .iter()
            .partition(|n| n.chars().nth(counter).unwrap() == '0');

        input = match (switch, _ones.len() >= _zeroes.len()) {
            (true, true) => _zeroes,
            (false, false) => _zeroes,
            _ => _ones,
        };

        if input.len() == 1 {
            break;
        }
        counter += 1
    }
    return input[0];
}
fn main() {
    let data = fs::read_to_string("./dummy_data.txt").expect("Unable to read file");
    let mut parsed_data: Vec<&str> = data.split('\n').collect();

    println!("{:?}", isize::from_str_radix(extract(parsed_data.clone(), false), 2).unwrap() * isize::from_str_radix(extract(parsed_data.clone(), true), 2).unwrap())
}
