use homework_3::gauss::gauss;
use homework_3::par_gauss1::parallel_gauss_1;
use homework_3::par_gauss2::parallel_gauss_2;
use homework_3::par_gauss3::parallel_gauss_3;
use homework_3::par_gauss4::parallel_gauss_4;


#[test]
fn test_gauss() {
    let a = vec![
        vec![2.0, 1.0, -1.0],
        vec![-3.0, -1.0, 2.0],
        vec![-2.0, 1.0, 2.0],
    ];
    let b = vec![8.0, -11.0, -3.0];

    let solution1 = gauss(a.clone(), b.clone());

    match solution1 {
        Ok(result) => {
            assert_eq!(result, vec![2.0, 3.0, -0.9999999999999999]);
        },
        Err(err) => {
            panic!("Error: {}", err);
        }
    }
}

#[test]
fn test_pargauss1() {
    let a = vec![
        vec![2.0, 1.0, -1.0],
        vec![-3.0, -1.0, 2.0],
        vec![-2.0, 1.0, 2.0],
    ];
    let b = vec![8.0, -11.0, -3.0];

    let solution = parallel_gauss_1(a.clone(), b.clone());

    match solution {
        Ok(result) => {
            assert_eq!(result, vec![2.0, 3.0, -0.9999999999999999]);
        },
        Err(err) => {
            panic!("Error: {}", err);
        }
    }
}
#[test]
fn test_pargauss2() {
    let a = vec![
        vec![2.0, 1.0, -1.0],
        vec![-3.0, -1.0, 2.0],
        vec![-2.0, 1.0, 2.0],
    ];
    let b = vec![8.0, -11.0, -3.0];

    let solution = parallel_gauss_2(a.clone(), b.clone());

    match solution {
        Ok(result) => {
            assert_eq!(result, vec![2.0, 3.0, -0.9999999999999999]);
        },
        Err(err) => {
            panic!("Error: {}", err);
        }
    }
}

#[test]
fn test_pargauss3() {
    let a = vec![
        vec![2.0, 1.0, -1.0],
        vec![-3.0, -1.0, 2.0],
        vec![-2.0, 1.0, 2.0],
    ];
    let b = vec![8.0, -11.0, -3.0];

    let solution = parallel_gauss_3(a.clone(), b.clone());

    match solution {
        Ok(result) => {
            assert_eq!(result, vec![2.0, 3.0, -0.9999999999999999]);
        },
        Err(err) => {
            panic!("Error: {}", err);
        }
    }
}
#[test]
fn test_pargauss4() {
    let a = vec![
        vec![2.0, 1.0, -1.0],
        vec![-3.0, -1.0, 2.0],
        vec![-2.0, 1.0, 2.0],
    ];
    let b = vec![8.0, -11.0, -3.0];

    let solution = parallel_gauss_4(a.clone(), b.clone());

    match solution {
        Ok(result) => {
            assert_eq!(result, vec![2.0, 3.0, -0.9999999999999999]);
        },
        Err(err) => {
            panic!("Error: {}", err);
        }
    }
}
