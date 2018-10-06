extern crate rand;
extern crate rayon;
use rand::Rng;
use rayon::prelude::*;

#[macro_use]
extern crate criterion;
use criterion::{Criterion, Fun};

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn add_two_inline_return(v: &[i32]) -> Vec<i32> {
    let r = add_two_inline_return_inner(v);
    r
}

pub fn add_two_inline_return_inner(v: &[i32]) -> Vec<i32> {
    let mut res: Vec<i32> = vec![0; v.len()];
    for i in 0..v.len() {
        res[i] = v[i] + 2;
    }
    res
}

pub fn add_two_iter_return(v: &[i32]) -> Vec<i32> {
    let r = add_two_iter_return_inner(v);
    r
}

pub fn add_two_iter_return_inner(v: &[i32]) -> Vec<i32> {
    v.iter().map(|x| x + 2).collect()
}

pub fn add_two_iter_mut_outparam(v: &mut [i32]) {
    v.iter_mut().for_each(|x| *x += 2);
}

pub fn add_two_par_iter_mut_outparam(v: &mut [i32]) {
    v.par_iter_mut().for_each(|x| *x += 2);
}

pub fn add_two_par_iter_return(v: &[i32]) -> Vec<i32> {
    let r = add_two_par_iter_return_inner(v);
    r
}

pub fn add_two_par_iter_return_inner(v: &[i32]) -> Vec<i32> {
    v.par_iter().map(|x| x + 2).collect()
}

pub fn add_two_par_intoiter_return(v: &[i32]) -> Vec<i32> {
    let r = add_two_par_intoiter_return_inner(v);
    r
}

pub fn add_two_par_intoiter_return_inner(v: &[i32]) -> Vec<i32> {
    v.into_par_iter().map(|x| x + 2).collect()
}

pub fn add_two_else_add_three_return(v: &[i32]) -> Vec<i32> {
    let r = add_two_else_add_three_return_inner(v);
    r
}

pub fn add_two_else_add_three_return_inner(v: &[i32]) -> Vec<i32> {
    v.into_iter()
        .map(|&x| if x <= 5 { x + 2 } else { x + 3 })
        .collect()
}

pub fn add_two_else_add_three_par_return(v: &[i32]) -> Vec<i32> {
    let r = add_two_else_add_three_par_return_inner(v);
    r
}

pub fn add_two_else_add_three_par_return_inner(v: &[i32]) -> Vec<i32> {
    v.into_par_iter()
        .map(|&x| if x <= 5 { x + 2 } else { x + 3 })
        .collect::<Vec<i32>>()
}

// benchmarks

pub fn simd(c: &mut Criterion) {
    let simd_inline = Fun::new("simd_inline", |b, v: &Vec<i32>| {
        b.iter(|| add_two_inline_return(v))
    });
    let simd_iter = Fun::new("simd_iter", |b, v: &Vec<i32>| {
        b.iter(|| add_two_iter_return(v))
    });

    let simd_iter_mut = Fun::new("simd_iter_mut", |b, v: &Vec<i32>| {
        let mut v = v.clone();
        b.iter(|| add_two_iter_mut_outparam(&mut v))
    });

    let simd_par_iter_mut = Fun::new("simd_par_iter_mut", |b, v: &Vec<i32>| {
        let mut v = v.clone();
        b.iter(|| add_two_par_iter_mut_outparam(&mut v))
    });
    let simd_par_iter = Fun::new("simd_par_iter", |b, v: &Vec<i32>| {
        b.iter(|| add_two_par_iter_return(v))
    });

    let simd_par_intoiter = Fun::new("simd_par_intoiter", |b, v: &Vec<i32>| {
        b.iter(|| add_two_par_intoiter_return(v))
    });

    let simd_branching_no_sorting = Fun::new("With if-else checks", |b, v: &Vec<i32>| {
        b.iter(|| add_two_else_add_three_return(v))
    });

    let simd_branching_no_sorting_par =
        Fun::new("With if-else checks - parallel", |b, v: &Vec<i32>| {
            b.iter(|| add_two_else_add_three_par_return(v))
        });

    let functions = vec![
        simd_inline,
        simd_iter,
        simd_iter_mut,
        simd_par_iter_mut,
        simd_par_iter,
        simd_par_intoiter,
        simd_branching_no_sorting,
        simd_branching_no_sorting_par,
    ];

    // let test_vector: Vec<i32> = vec![1; 100_000];
    use rand::distributions::Standard;
    let mut rng = rand::thread_rng();
    let test_vector: Vec<i32> = rng
        .sample_iter(&Standard)
        .take(100_000)
        .map(|x: i32| (x % 10) + 1)
        .collect();

    c.bench_functions("Simdings", functions, test_vector);
}

criterion_group!(benches, simd);
criterion_main!(benches);
