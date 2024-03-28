pub fn sum(matrix: [[f64; 12]; 12]) -> (f64, i32) {
    let mut sum: f64 = 0.0;
    let mut elems: i32 = 0;
    let mut column: usize = matrix.len();
    
    for row in 1..matrix.len() - 1 {
        match row {
            1..=5 => column -= 1,
            7..=10 => column += 1,
            _ => (),
        }

        for i in column..matrix.len() {
            sum += matrix[row][i];
            elems += 1;
        }
    }

    (sum, elems)
}

pub fn avg(matrix: [[f64; 12]; 12]) -> f64 {
    let (sum, elems) = sum(matrix);

    sum / elems as f64
}
