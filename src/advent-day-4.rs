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
        let _ = self.array.append(
            Axis(0),
            ArrayView::from(&append).into_shape((1, 5)).unwrap(),
        );
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
    let mut parsed_data: Vec<&str> = data.split("\n").filter(|x| !x.is_empty()).collect();
    let numbers: Vec<i8> = parsed_data.clone()[0]
        .split(",")
        .map(|x| x.parse::<i8>().unwrap())
        .collect();
        
    parsed_data.remove(0);
    let mut current = Matrix::new();

    for row in parsed_data {
        let parsed_row: Vec<i32> = row
            .replace(" ", ",")
            .replace(",,", ",")
            .trim_start_matches(",")
            .split(",")
            .map(|x| x.trim_start().parse::<i32>().unwrap())
            .collect();
        
        current.append(parsed_row);

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
            matrix.array.map_inplace(|x| {
                if x == &(number as i32) {
                    *x = 100;
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
    println!("First winner: {:?}", (winners[0].array.sum() - winners[0].increment) * (winners[0].winning_number as i32));
    println!("Last Winner: {:?}", (last_winner.array.sum() - last_winner.increment) * (last_winner.winning_number as i32));
}
