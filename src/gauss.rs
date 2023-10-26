pub fn gauss(mut a: Vec<Vec<f64>>, mut b: Vec<f64>) -> Result<Vec<f64>,String> {
    let n = b.len();

    // Forward Elimination (Eliminate variables below the diagonal)
    for pivot_row in 0..n {
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
            return Err("Matrix is singular or poorly conditioned.".to_string());
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
    }

    // Backward Substitution (Solve for variables)
    let mut x = vec![0.0; n];
    for i in (0..n).rev() {
        x[i] = b[i];
        for j in (i + 1)..n {
            x[i] -= a[i][j] * x[j];
        }
    }

    Ok(x)
}