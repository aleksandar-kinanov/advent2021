use std::fs;
fn extract(mut input: Vec<&str>, switch: bool) -> &str{
    let mut _ones: Vec<&str> = Vec::new();
    let mut _zeroes: Vec<&str> = Vec::new();
    let mut counter = 0;
    let break_point= input.clone().iter().nth(0).unwrap().len();

    while counter < break_point{
        _ones.clear();
        _zeroes.clear();

        for item in input.clone(){
            let current = item.chars().nth(counter).unwrap();
            match current {
                '1' => _ones.push(item),
                '0' => _zeroes.push(item),
                _ => ()
                
            }
        }
        input = match 1 {
            _ if _ones.len() >= _zeroes.len() && switch => _zeroes.clone(),
            _ if _ones.len() < _zeroes.len() && switch => _ones.clone(),
            _ if _ones.len() >= _zeroes.len() && !switch => _ones.clone(),
            _ if _ones.len() < _zeroes.len() && !switch => _zeroes.clone(),
            _ => Vec::new()
        };
        if input.len() == 1{
            break;
        }
        counter += 1
    }
    return input[0];
}
fn main (){
    let data = fs::read_to_string("./dummy_data.txt").expect("Unable to read file");
    let mut parsed_data: Vec<&str> = data.split('\n').collect();
    parsed_data.sort();
    println!("{:?}", isize::from_str_radix(extract(parsed_data.clone(), false), 2).unwrap() * isize::from_str_radix(extract(parsed_data.clone(), true), 2).unwrap())
}