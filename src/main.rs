use std::fs;
use std::collections::HashMap;
#[derive(Debug)]
struct Test {
    nested_hashmap: HashMap<String,HashMap<String, i32>>,
  }
fn main (){
    let data = fs::read_to_string("./dummy_data.txt").expect("Unable to read file");
    let parsed_data: Vec<&str> = data.split('\n').collect();
    let radix = parsed_data[0].len();

    let mut test = Test { nested_hashmap: HashMap::new() };
    for item in parsed_data{
        let char_vec: Vec<char> = item.chars().collect();
        for (i, el) in char_vec.iter().enumerate() {
            match i.to_string().as_str() {
                _ => test.nested_hashmap
                    .entry(format!("Index{}", char::from_digit(i as u32, radix as u32).unwrap()).to_string())
                    .or_insert(HashMap::new())
                    .entry(format!("{}", el).to_string())
                    .and_modify(|target| *target += 1)
                    .or_insert(1)
            };
        }
    }
    let mut bin_rep: String = "".to_owned();
    let mut sorted: Vec<_> = test.nested_hashmap.into_iter().collect();

    sorted.sort_by(|x,y| x.0.cmp(&y.0));
    for (_,v) in sorted {

        let current_reuslt = v
        .iter()
        .max_by(|a, b| a.1.cmp(&b.1))
        .map(|(k, _v)| k);

        bin_rep.push_str(current_reuslt.unwrap())
    }

    let parsed_binary = isize::from_str_radix(&bin_rep, 2).unwrap();
    let inversed = 0xfff & !parsed_binary;
    let result = parsed_binary * inversed;
    println!("{}", result); 
}