use rayon::prelude::*;
use std::sync::{Arc, Mutex};

pub fn parallel_gauss_3(a: Vec<Vec<f64>>, b: Vec<f64>) -> Result<Vec<f64>, String> {
    let n = b.len();

    let a = Arc::new(Mutex::new(a)); // Wrap 'a' with Arc and Mutex
    let b = Arc::new(Mutex::new(b)); // Wrap 'b' with Arc and Mutex

    // Forward Elimination (Eliminate variables below the diagonal)
    (0..n).into_par_iter().for_each(|pivot_row| {
        let a = Arc::clone(&a);
        let b = Arc::clone(&b);


        let mut a = a.lock().unwrap(); // Lock Mutex for 'a'
        let mut b = b.lock().unwrap(); // Lock Mutex for 'b'

        // Find the pivot element
        let mut pivot_index = pivot_row;
        for i in (pivot_row + 1)..n {
            if a[i][pivot_row].abs() > a[pivot_index][pivot_row].abs() {
                pivot_index = i;
            }
        }

        // Swap rows if necessary
        if pivot_index != pivot_row {
            a.swap(pivot_row, pivot_index);
            b.swap(pivot_row, pivot_index);
        }

        let pivot_value = a[pivot_row][pivot_row];

        if pivot_value == 0.0 {
            // Return an error early if the matrix is singular or poorly conditioned
            // return Err("Matrix is singular or poorly conditioned.".to_string());
        }

        // Normalize the pivot row
        for j in pivot_row..n {
            a[pivot_row][j] /= pivot_value;
        }
        b[pivot_row] /= pivot_value;

        // Eliminate variables below the pivot row
        for i in (pivot_row + 1)..n {
            let factor = a[i][pivot_row];
            for j in pivot_row..n {
                a[i][j] -= factor * a[pivot_row][j];
            }
            b[i] -= factor * b[pivot_row];
        }
    });

    // Backward Substitution (Solve for variables)
    let mut x = vec![0.0; n];
    for i in (0..n).rev() {
        x[i] = b.lock().unwrap()[i];
        for j in (i + 1)..n {
            x[i] -= a.lock().unwrap()[i][j] * x[j];
        }
    }

    Ok(x)
}