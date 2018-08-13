extern crate rayon;
use rayon::prelude::*;

#[macro_use]
extern crate criterion;
use criterion::{Criterion, Fun};

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn add_two_via_func(v: &mut Vec<i32>) {
    for i in 0..v.len() {
        v[i] = add_two(v[i]);
    }
}

pub fn add_two_via_lambda<T>(v: &mut Vec<i32>, f: T)
where
    T: Fn(i32) -> i32,
{
    // let f = |x| x + 2;
    for i in 0..v.len() {
        v[i] = f(v[i]);
    }
}

pub fn add_two_inline(v: &mut Vec<i32>) {
    for i in 0..v.len() {
        v[i] = v[i] + 2;
    }
}

pub fn add_two_iter(v: &Vec<i32>) -> Vec<i32> {
    v.iter().map(|x| x + 2).collect()
}

pub fn add_two_par_iter_mut(v: &mut Vec<i32>) {
    v.par_iter_mut().for_each(|x| *x += 2);
}

pub fn add_two_par_iter(v: &Vec<i32>) -> Vec<i32> {
    v.par_iter().map(|x| x + 2).collect()
}

pub fn add_two_par_intoiter(v: &Vec<i32>) -> Vec<i32> {
    v.into_par_iter().map(|x| x + 2).collect()
}

// benchmarks

pub fn simd(c: &mut Criterion) {
    let simd_via_func = Fun::new("simd_via_func", |b, v: &Vec<i32>| {
        let mut v_mut = v.clone();
        b.iter(move || add_two_via_func(&mut v_mut))
    });
    let simd_via_lambda = Fun::new("simd_via_lambda", |b, v: &Vec<i32>| {
        let mut v_mut = v.clone();
        let f = |x| x + 2;
        b.iter(|| add_two_via_lambda(&mut v_mut, f))
    });
    let simd_inline = Fun::new("simd_inline", |b, v: &Vec<i32>| {
        let mut v_mut = v.clone();
        b.iter(|| add_two_inline(&mut v_mut))
    });
    let simd_iter = Fun::new("simd_iter", |b, v: &Vec<i32>| b.iter(|| add_two_iter(v)));
    let simd_par_iter_mut = Fun::new("simd_par_iter_mut", |b, v: &Vec<i32>| {
        let mut res = v.clone();
        b.iter(move || add_two_par_iter_mut(&mut res))
    });
    let simd_par_iter = Fun::new("simd_par_iter", |b, v: &Vec<i32>| {
        b.iter(|| add_two_par_iter(v))
    });

    let simd_par_intoiter = Fun::new("simd_par_intoiter", |b, v: &Vec<i32>| {
        b.iter(|| add_two_par_intoiter(v))
    });
    let functions = vec![
        simd_via_func,
        simd_via_lambda,
        simd_inline,
        simd_iter,
        simd_par_iter_mut,
        simd_par_iter,
        simd_par_intoiter,
    ];
    let test_vector: Vec<i32> = vec![1; 1_000_000];

    c.bench_functions("Simdings", functions, test_vector);
}

criterion_group!(benches, simd);
criterion_main!(benches);
