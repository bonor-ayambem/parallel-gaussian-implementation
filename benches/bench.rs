#![feature(test)]

extern crate test;
use test::Bencher;
use rand::Rng;

use homework_3::gauss::gauss;
use homework_3::par_gauss1::parallel_gauss_1;
use homework_3::par_gauss2::parallel_gauss_2;
use homework_3::par_gauss3::parallel_gauss_3;
use homework_3::par_gauss4::parallel_gauss_4;

fn generate_random_matrices(size: usize) -> (Vec<Vec<f64>>, Vec<f64>) {
    let mut rng = rand::thread_rng();

    let mut a = Vec::new();
    for _ in 0..size {
        let row = (0..size).map(|_| rng.gen_range(-10.0..10.0)).collect();
        a.push(row);
    }

    let b: Vec<f64> = (0..size).map(|_| rng.gen_range(-10.0..10.0)).collect();

    (a, b)
}

#[bench]
fn gauss_256(b: &mut Bencher) {
    b.iter(|| {
        let (a,b) = generate_random_matrices(16);
        let _ = gauss(a.clone(), b.clone());
    });
}

#[bench]
fn gauss_512(b: &mut Bencher) {
    b.iter(|| {
        let (a,b) = generate_random_matrices(23);
        let _ = gauss(a.clone(), b.clone());
    });
}

#[bench]
fn gauss_1024(b: &mut Bencher) {
    b.iter(|| {
        let (a,b) = generate_random_matrices(32);
        let _ =gauss(a.clone(), b.clone());
    });
}

#[bench]
fn gauss_2048(b: &mut Bencher) {
    b.iter(|| {
        let (a,b) = generate_random_matrices(45);
        let _ = gauss(a.clone(), b.clone());
    });
}

#[bench]
fn gauss_4096(b: &mut Bencher) {
    b.iter(|| {
        let (a,b) = generate_random_matrices(64);
        let _ = gauss(a.clone(), b.clone());
    });
}

#[bench]
fn par1_256(b: &mut Bencher) {
    b.iter(|| {
        let (a,b) = generate_random_matrices(16);
        let _ = parallel_gauss_1(a.clone(), b.clone());
    });
}

#[bench]
fn par1_512(b: &mut Bencher) {
    b.iter(|| {
        let (a,b) = generate_random_matrices(23);
        let _ = parallel_gauss_1(a.clone(), b.clone());
    });
}

#[bench]
fn par1_1024(b: &mut Bencher) {
    b.iter(|| {

        let (a,b) = generate_random_matrices(32);
        let _ = parallel_gauss_1(a.clone(), b.clone());
    });
}

#[bench]
fn par1_2048(b: &mut Bencher) {
    b.iter(|| {
        let (a,b) = generate_random_matrices(45);
        let _ = parallel_gauss_1(a.clone(), b.clone());
    });
}

#[bench]
fn par1_4096(b: &mut Bencher) {
    b.iter(|| {
        let (a,b) = generate_random_matrices(64);
        let _ = parallel_gauss_2(a.clone(), b.clone());
    });
}

#[bench]
fn par2_256(b: &mut Bencher) {
    b.iter(|| {
        let (a,b) = generate_random_matrices(16);
        let _ = parallel_gauss_2(a.clone(), b.clone());
    });
}

#[bench]
fn par2_512(b: &mut Bencher) {
    b.iter(|| {
        let (a,b) = generate_random_matrices(23);
        let _ = parallel_gauss_2(a.clone(), b.clone());
    });
}

#[bench]
fn par2_1024(b: &mut Bencher) {
    b.iter(|| {
        let (a,b) = generate_random_matrices(32);
        let _ = parallel_gauss_2(a.clone(), b.clone());
    });
}

#[bench]
fn par2_2048(b: &mut Bencher) {
    b.iter(|| {
        let (a,b) = generate_random_matrices(45);
        let _ = parallel_gauss_2(a.clone(), b.clone());
    });
}

#[bench]
fn par2_4096(b: &mut Bencher) {
    b.iter(|| {
        let (a,b) = generate_random_matrices(64);
        let _ = parallel_gauss_2(a.clone(), b.clone());
    });
}

#[bench]
fn par3_256(b: &mut Bencher) {
    b.iter(|| {
        let (a,b) = generate_random_matrices(16);
        let _ = parallel_gauss_3(a.clone(), b.clone());
    });
}

#[bench]
fn par3_512(b: &mut Bencher) {
    b.iter(|| {
        let (a,b) = generate_random_matrices(23);
        let _ = parallel_gauss_3(a.clone(), b.clone());
    });
}

#[bench]
fn par3_1024(b: &mut Bencher) {
    b.iter(|| {
        let (a,b) = generate_random_matrices(32);
        let _ = parallel_gauss_3(a.clone(), b.clone());
    });
}

#[bench]
fn par3_2048(b: &mut Bencher) {
    b.iter(|| {
        let (a,b) = generate_random_matrices(45);
        let _ = parallel_gauss_3(a.clone(), b.clone());
    });
}

#[bench]
fn par3_4096(b: &mut Bencher) {
    b.iter(|| {
        let (a,b) = generate_random_matrices(64);
        let _ = parallel_gauss_3(a.clone(), b.clone());
    });
}

#[bench]
fn par4_256(b: &mut Bencher) {
    b.iter(|| {
        let (a,b) = generate_random_matrices(16);
        let _ = parallel_gauss_4(a.clone(), b.clone());
    });
}

#[bench]
fn par4_512(b: &mut Bencher) {
    b.iter(|| {
        let (a,b) = generate_random_matrices(23);
        let _ = parallel_gauss_4(a.clone(), b.clone());
    });
}

#[bench]
fn par4_1024(b: &mut Bencher) {
    b.iter(|| {
        let (a,b) = generate_random_matrices(32);
        let _ = parallel_gauss_4(a.clone(), b.clone());
    });
}

#[bench]
fn par4_2048(b: &mut Bencher) {
    b.iter(|| {
        let (a,b) = generate_random_matrices(45);
        let _ = parallel_gauss_4(a.clone(), b.clone());
    });
}

#[bench]
fn par4_4096(b: &mut Bencher) {
    b.iter(|| {
        let (a,b) = generate_random_matrices(64);
        let _ = parallel_gauss_4(a.clone(), b.clone());
    });
}