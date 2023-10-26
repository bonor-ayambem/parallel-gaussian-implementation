use homework_3::gauss::gauss;
use homework_3::par_gauss3::parallel_gauss_3;


fn main() {
    // Example usage:
    let a = vec![
        vec![2.0, 1.0, -1.0],
        vec![-3.0, -1.0, 2.0],
        vec![-2.0, 1.0, 2.0],
    ];
    let b = vec![8.0, -11.0, -3.0];

    let solution = gauss(a.clone(), b.clone());
    let solution2 = parallel_gauss_3(a.clone(), b.clone());

    println!("Solution: {:?}", solution);
    println!("Parallel Solution 1: {:?}", solution2);
}

// crossbeam: send rows to threads and receive to use on other threads
// test for different sizes of matrices