#![feature(test)]
extern crate num_cpus;
extern crate rayon;
extern crate test;

use rayon::prelude::*;
use std::thread;

static V1: &[u32] = &[3; 1_000_003];
static V2: &[u32] = &[2; 1_000_003];

pub fn scalar_product(vec1: &[u32], vec2: &[u32]) -> u32 {
    vec1.iter().zip(vec2.iter()).map(|(x1, x2)| x1 * x2).sum()
}

pub fn scalar_product_for(vec1: &[u32], vec2: &[u32]) -> u32 {
    let n = vec1.len();
    let mut sum = 0;
    for i in 0..n {
        sum += vec1[i] * vec2[i];
    }
    sum
}

pub fn rayon_product(vec1: &[u32], vec2: &[u32]) -> u32 {
    vec1.par_iter()
        .zip(vec2.par_iter())
        .map(|(x1, x2)| x1 * x2)
        .sum()
}

pub fn par_scalar_product(vec1: &'static [u32], vec2: &'static [u32]) -> u32 {
    let n_threads = num_cpus::get();
    let n = vec1.len() / n_threads;
    let mut sum = 0;
    let mut threads = Vec::new();
    for i in 0..n_threads {
        threads.push(thread::spawn(move || {
            scalar_product(&vec1[i * n..(i + 1) * n], &vec2[i * n..(i + 1) * n])
        }));
    }
    sum += scalar_product(&vec1[n_threads * n..], &vec2[n_threads * n..]);
    for thread in threads {
        sum += thread.join().unwrap();
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    pub fn test_products() {
        assert_eq!(rayon_product(&V1, &V2), scalar_product(&V1, &V2));
        assert_eq!(scalar_product_for(&V1, &V2), scalar_product(&V1, &V2));
        assert_eq!(par_scalar_product(&V1, &V2), scalar_product(&V1, &V2));
    }
    #[bench]
    pub fn bench_rayon_product(b: &mut Bencher) {
        b.iter(|| rayon_product(&V1, &V2));
    }

    #[bench]
    pub fn bench_scalar_product(b: &mut Bencher) {
        b.iter(|| scalar_product(&V1, &V2));
    }

    #[bench]
    pub fn bench_scalar_product_for(b: &mut Bencher) {
        b.iter(|| scalar_product_for(&V1, &V2));
    }

    #[bench]
    pub fn bench_par_scalar_product(b: &mut Bencher) {
        b.iter(|| par_scalar_product(&V1, &V2));
    }
}
