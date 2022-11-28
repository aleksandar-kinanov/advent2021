use ndarray::{Array2, ArrayView, Axis};
use std::fs;
#[derive(Debug, Clone)]
struct Matrix {
    array: Array2<i32>,
    increment: i32,
    winning_number: i8,
    is_winner: bool
}

impl Matrix {
    fn append(&mut self, append: Vec<i32>) {
        self.array.append(Axis(0),ArrayView::from(&append).into_shape((1, 5)).unwrap());
    }

    fn new() -> Matrix {
        Matrix {
            array: Array2::<i32>::default((0, 5)),
            increment: 0,
            winning_number: 0,
            is_winner: false
        }
    }
    fn is_full(&self) -> bool {
        self.array.len_of(Axis(0)) == 5
    }
}
fn main() {
    let mut matrices: Vec<Matrix> = Vec::new();
    let mut winners: Vec<Matrix> = Vec::new();
    let data = fs::read_to_string("./dummy_data.txt").expect("Unable to read file");
    // Remove empty lines
    let mut parsed_data: Vec<&str> = data.split("\n").filter(|x| !x.is_empty()).collect();
    // Get 0th element of the loaded data and load as ints
    let numbers: Vec<i8> = parsed_data.clone()[0]
        .split(",")
        .map(|x| x.parse::<i8>().unwrap())
        .collect();
        
    parsed_data.remove(0);
    let mut current = Matrix::new();

    // Iterate over every row and remove all unnecessary empty spaces and new lines and replace them with ','
    for row in parsed_data {
        let parsed_row: Vec<i32> = row
            .replace(" ", ",")
            .replace(",,", ",")
            .trim_start_matches(",")
            .split(",")
            .map(|x| x.trim_start().parse::<i32>().unwrap())
            .collect();
        
        // Append the parsed arrays 1 by 1 to the Matrix object
        current.append(parsed_row);
        // Stop adding elements once the matrix is 5x5
        if current.is_full() {
            matrices.push(current.clone());
            current = Matrix::new();
        }
    }
    for number in numbers {
        if winners.len() == matrices.len() {
            break;
        }
        for matrix in &mut matrices {
            if matrix.is_winner {
                continue
            }
            // Replace every matching element with 100 as the matices have upper bound of elements of 99
            matrix.array.map_inplace(|x| {
                if x == &(number as i32) {
                    *x = 100;
                    // Add incremented values so they can be removed from the total sum of the matrix
                    matrix.increment += 100;
                }
            });
            if matrix.array.sum_axis(Axis(0)).into_raw_vec().contains(&500) || matrix.array.sum_axis(Axis(1)).into_raw_vec().contains(&500){ 
                matrix.winning_number = number;
                matrix.is_winner = true;
                winners.push(matrix.clone());
            }
        }
    }
    let last_winner = winners.last().unwrap();
    // Get the Matrix that was first in getting a 'bingo' - entire column/row of 100s
    println!("First winner: {:?}", (winners[0].array.sum() - winners[0].increment) * (winners[0].winning_number as i32));
    // Get the Matrix that was last in getting a 'bingo' - entire column/row of 100s
    println!("Last Winner: {:?}", (last_winner.array.sum() - last_winner.increment) * (last_winner.winning_number as i32));
}
