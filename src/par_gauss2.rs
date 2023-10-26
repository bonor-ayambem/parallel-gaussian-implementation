use rayon::prelude::*;
use std::sync::{Arc, Mutex};

pub fn parallel_gauss_2(a: Vec<Vec<f64>>, b: Vec<f64>) -> Result<Vec<f64>, String> {
    let n = b.len();
    let a = Arc::new(Mutex::new(a));
    let b = Arc::new(Mutex::new(b));

    // Forward Elimination (Eliminate variables below the diagonal)
    for pivot_row in 0..n {
        // // Find the pivot element
        // let mut pivot_index = pivot_row;
        // for i in (pivot_row + 1)..n {
        //     if a[i][pivot_row].abs() > a[pivot_index][pivot_row].abs() {
        //         pivot_index = i;
        //     }
        // }
        let mut pivot_index = pivot_row;

        // Parallelize this section to find the pivot element
        pivot_index = (pivot_row..n)
            .into_par_iter()
            .map(|i| (i, a.lock().unwrap()[i][pivot_row].abs()))
            .reduce(
                || (pivot_row, 0.0),
                |(index1, max1), (index2, max2)| {
                    if max1 > max2 {
                        (index1, max1)
                    } else {
                        (index2, max2)
                    }
                },
            )
            .0;

        // Swap rows if necessary
        if pivot_index != pivot_row {
            a.lock().unwrap().swap(pivot_row, pivot_index);
            b.lock().unwrap().swap(pivot_row, pivot_index);
        }

        let pivot_value = a.lock().unwrap()[pivot_row][pivot_row];

        if pivot_value == 0.0 {
            return Err("Matrix is singular or poorly conditioned.".to_string());
        }

        a.lock().unwrap()[pivot_row].par_iter_mut().for_each(|element| *element /= pivot_value);
        b.lock().unwrap()[pivot_row] /= pivot_value;

        // // Normalize the pivot row
        // for j in pivot_row..n {
        //     a[pivot_row][j] /= pivot_value;
        // }
        // b[pivot_row] /= pivot_value;

        // Parallelize the elimination loop
        (pivot_row + 1..n).into_par_iter().for_each(|i| {
            let factor = a.lock().unwrap()[i][pivot_row];
            let mut a_inner = a.lock().unwrap();
            let mut b_inner = b.lock().unwrap();

            for j in 0..n {
                a_inner[i][j] -= factor * a_inner[pivot_row][j];
            }
            b_inner[i] -= factor * b_inner[pivot_row];
        });

        // // Eliminate variables below the pivot row
        // for i in (pivot_row + 1)..n {
        //     let factor = a[i][pivot_row];
        //     for j in pivot_row..n {
        //         a[i][j] -= factor * a[pivot_row][j];
        //     }
        //     b[i] -= factor * b[pivot_row];
        // }
    }

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
