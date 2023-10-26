use std::thread;
use std::sync::{Arc, Mutex};

pub fn parallel_gauss_4(a: Vec<Vec<f64>>, b: Vec<f64>) -> Result<Vec<f64>, String> {
    let n = b.len();
    let a = Arc::new(Mutex::new(a));
    let b = Arc::new(Mutex::new(b));

    for pivot_row in 0..n {
        let mut pivot_index = pivot_row;

        // Find the pivot element
        for i in (pivot_row + 1)..n {
            let a_lock = a.lock().unwrap();
            if a_lock[i][pivot_row].abs() > a_lock[pivot_index][pivot_row].abs() {
                pivot_index = i;
            }
        }

        // Swap rows if necessary
        if pivot_index != pivot_row {
            let mut a_lock = a.lock().unwrap();
            let mut b_lock = b.lock().unwrap();
            a_lock.swap(pivot_row, pivot_index);
            b_lock.swap(pivot_row, pivot_index);
        }

        let pivot_value = {
            let a_lock = a.lock().unwrap();
            a_lock[pivot_row][pivot_row]
        };

        if pivot_value == 0.0 {
            return Err("Matrix is singular or poorly conditioned.".to_string());
        }

        // Normalize the pivot row
        {
            let mut a_lock = a.lock().unwrap();
            let mut b_lock = b.lock().unwrap();
            for j in pivot_row..n {
                a_lock[pivot_row][j] /= pivot_value;
            }
            b_lock[pivot_row] /= pivot_value;
        }

        // Eliminate variables below the pivot row in parallel
        let mut handles = vec![];
        for i in (pivot_row + 1)..n {
            let factor = {
                let a_lock = a.lock().unwrap();
                a_lock[i][pivot_row]
            };

            let a_clone = a.clone();
            let b_clone = b.clone();

            let handle = thread::spawn(move || {
                let mut a_lock = a_clone.lock().unwrap();
                let mut b_lock = b_clone.lock().unwrap();
                for j in pivot_row..n {
                    a_lock[i][j] -= factor * a_lock[pivot_row][j];
                }
                b_lock[i] -= factor * b_lock[pivot_row];
            });

            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }
    }

    // Backward Substitution (Solve for variables)
    let mut x = vec![0.0; n];
    for i in (0..n).rev() {
        x[i] = {
            // let a_lock = a.lock().unwrap();
            let b_lock = b.lock().unwrap();
            b_lock[i]
        };
        for j in (i + 1)..n {
            x[i] -= {
                let a_lock = a.lock().unwrap();
                a_lock[i][j] * x[j]
            };
        }
    }

    Ok(x)
}


// use std::thread;

// pub fn parallel_gauss_4(mut a: Vec<Vec<f64>>, mut b: Vec<f64>) -> Result<Vec<f64>, String> {
//     let n = b.len();

//     for pivot_row in 0..n {
//         let mut pivot_index = pivot_row;

//         // Find the pivot element
//         for i in (pivot_row + 1)..n {
//             if a[i][pivot_row].abs() > a[pivot_index][pivot_row].abs() {
//                 pivot_index = i;
//             }
//         }

//         // Swap rows if necessary
//         if pivot_index != pivot_row {
//             a.swap(pivot_row, pivot_index);
//             b.swap(pivot_row, pivot_index);
//         }

//         let pivot_value = a[pivot_row][pivot_row];

//         if pivot_value == 0.0 {
//             return Err("Matrix is singular or poorly conditioned.".to_string());
//         }

//         // Normalize the pivot row
//         for j in pivot_row..n {
//             a[pivot_row][j] /= pivot_value;
//         }
//         b[pivot_row] /= pivot_value;


//         // Eliminate variables below the pivot row in parallel
//         let mut handles = vec![];
//         for i in (pivot_row + 1)..n {
//             let factor = a[i][pivot_row];
//             // let a_row = a[i].clone(); // Clone the row for parallel processing
//             // let b_i = b[i];

//             // // Clone the 'a' matrix for parallel processing
//             // let mut a_clone = a.clone();
//             // let mut b_clone = b.clone();

//             // let handle = thread::spawn(move || {
//             //     for j in pivot_row..n {
//             //         a_clone[i][j] -= factor * a_row[j];
//             //     }
//             //     b_clone[i] -= factor * b_i;
//             // });

//             let handle = thread::spawn(move || {
//                 for j in pivot_row..n {
//                     a[i][j] -= factor * a[pivot_row][j];
//                 }
//                 b[i] -= factor * b[pivot_row];
//             });

//             handles.push(handle);
//         }

//         for handle in handles {
//             handle.join().unwrap();
//         }
//     }

//     // Backward Substitution (Solve for variables)
//     let mut x = vec![0.0; n];
//     for i in (0..n).rev() {
//         x[i] = b[i];
//         for j in (i + 1)..n {
//             x[i] -= a[i][j] * x[j];
//         }
//     }

//     Ok(x)
// }
