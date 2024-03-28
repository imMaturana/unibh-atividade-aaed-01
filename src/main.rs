mod matrix;

use std::io::{self, Write};
use std::fs::File;
use serde_json;

fn main() {
    let file = File::open("data/matrix.json").unwrap(); // arquivo patrocinado pelo Gepeto.
    let matrix: [[f64; 12]; 12] = serde_json::from_reader(file).unwrap();
    let mut matrix_str: [[&str; 12]; 12] = [["."; 12]; 12];
    
    loop {
        let mut input = String::new();

        print!("Digite 'S' para somar e 'M' para exibir a média: ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut input)
            .expect("Erro ao ler input.");
        println!();

        let op = input.trim().to_uppercase();

        match op.as_str() {
            "S" => {
                let (sum, _) = matrix::sum(matrix);

                println!("Soma: {:.2}", sum);
            },
            "M" => {
                let avg = matrix::avg(matrix);

                println!("Média: {:.2}", avg);
            },
            _ => break
        }

        println!("\nRepresentação da Matriz.");
        display_matrix(&mut matrix_str);
    }
}

// mesma brisa da função matrix::sum().
fn display_matrix(matrix: &mut [[&str; 12]; 12]) {
    let mut column: usize = matrix.len();
    
    for row in 1..matrix.len() - 1 {
        match row {
            1..=5 => column -= 1,
            7..=10 => column += 1,
            _ => (),
        }

        for i in column..matrix.len() {
            matrix[row][i] = "X";
        }
    }

    for i in 0..matrix.len() {
        for j in 0..matrix.len() {
            print!("{} ", matrix[i][j]);
        }

        println!();
    }
}
