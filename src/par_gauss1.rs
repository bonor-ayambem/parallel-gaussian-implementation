use rayon::prelude::*;
use std::sync::{Arc, Mutex};

pub fn parallel_gauss_1(mut a: Vec<Vec<f64>>, mut b: Vec<f64>) -> Result<Vec<f64>, String> {
    let n = b.len();

    let x = Arc::new(Mutex::new(vec![0.0; n])); // Wrap 'x' with Arc and Mutex

    // let a = Arc::new(Mutex::new(a));
    // let b = Arc::new(Mutex::new(b));

    // Forward Elimination (Eliminate variables below the diagonal)
    for pivot_row in 0..n {
        // let a = Arc::clone(&a);
        // let b = Arc::clone(&b);
        // let x = Arc::clone(&x);

        // let mut a = a.lock().unwrap(); // Lock Mutex for 'a'
        // let mut b = b.lock().unwrap(); // Lock Mutex for 'b'

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
        for i in pivot_row + 1..n {
            // ((pivot_row + 1)..n).into_par_iter().for_each(|i| {
            let factor = a[i][pivot_row];
            for j in pivot_row..n {
                a[i][j] -= factor * a[pivot_row][j];
            }
            b[i] -= factor * b[pivot_row];
        }
    }

    // Backward Substitution (Solve for variables)
    // Collect the reversed range into a vector for parallel iteration
    let reversed_range: Vec<usize> = (0..n).rev().collect();
    reversed_range.into_par_iter().for_each(|i| {
        let mut x = x.lock().unwrap();
        x[i] = b[i];
        for j in (i + 1)..n {
            x[i] -= a[i][j] * x[j];
        }
    });

    // Extract the result from 'x' and return it
    let result = x.lock().unwrap().to_vec();
    Ok(result)
}
