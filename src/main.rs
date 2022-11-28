use itertools::Itertools;
use std::fs;

use ndarray::{Array2, ArrayBase, Dim, ViewRepr};

fn create_line(
    mut mutable_axis: ArrayBase<ViewRepr<&mut i32>, Dim<[usize; 1]>>,
    mut start: usize,
    mut end: usize,
) {
    if end < start {
        (start, end) = (end, start)
    }
    for i in start..=end {
        mutable_axis[i] += 1;
    }
}

fn diagonal(){
    println!("diagonal")
}

fn main() {
    let mut matrix = Array2::<i32>::zeros((1000, 1000));
    let data = fs::read_to_string("./dummy_data.txt").expect("Unable to read file");
    let array: Vec<usize> = data
        .replace(" -> ", ",")
        .replace("\n", ",")
        .split(",")
        .map(|n| n.parse().unwrap())
        .collect();
    
    for item in array.chunks(4) {
        let (x1, y1, x2, y2) = item.iter().next_tuple().unwrap();
        match (x1 == x2, y1 == y2) {
            (true, false) => create_line(matrix.row_mut(*x1), *y1, *y2),
            (false, true) => create_line(matrix.column_mut(*y1), *x1, *x2),
            _ => println!("Should skip"),
        }
    }
    let mut counter = 0;
    matrix.for_each(|x| {
        if *x > 1 {
            counter += 1
        }
    });
    println!("{:?}", matrix);
}
