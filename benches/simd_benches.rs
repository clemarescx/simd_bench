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

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[target_feature(enable = "avx2")]
pub unsafe fn add_two_with_intrinsics_auto(v: &mut [i32]) {
    add_two_iter_mut_outparam(v)
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[target_feature(enable = "avx2")]
pub unsafe fn add_two_with_intrinsics_auto_par(v: &mut [i32]) {
    add_two_par_iter_mut_outparam(v)
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[target_feature(enable = "avx2")]
pub unsafe fn add_two_with_intrinsics_implicit(v: &mut [i32]) {
    use std::arch::x86_64::*;
    assert_eq!(v.len() % 8, 0);
    let p_count = v.len() / 8;
    for i in 0..p_count {
        let offset = i * 8;
        let p = _mm256_set_epi32(
            v[offset],
            v[offset + 1],
            v[offset + 2],
            v[offset + 3],
            v[offset + 4],
            v[offset + 5],
            v[offset + 6],
            v[offset + 7],
        );
        let inc = _mm256_set1_epi32(2);
        let res = _mm256_add_epi32(p, inc);
        _mm256_maskstore_epi32(
            v.as_mut_ptr().offset(offset as isize) as *mut _,
            _mm256_set1_epi32(0xffff),
            res,
        );
    }
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

    let simd_with_avx2_intrinsics_auto =
        Fun::new("With avx2 intrinsics - auto", |b, v: &Vec<i32>| {
            if is_x86_feature_detected!("avx2") {
                let mut v = v.clone();
                b.iter(|| return unsafe { add_two_with_intrinsics_auto(&mut v) });
            }
        });

    let simd_with_avx2_intrinsics_auto_par =
        Fun::new("With avx2 intrinsics - auto parallel", |b, v: &Vec<i32>| {
            if is_x86_feature_detected!("avx2") {
                let mut v = v.clone();
                b.iter(|| return unsafe { add_two_with_intrinsics_auto_par(&mut v) });
            }
        });

    let simd_with_avx2_intrinsics_explicit =
        Fun::new("With avx2 intrinsics - implicit", |b, v: &Vec<i32>| {
            if is_x86_feature_detected!("avx2") {
                let mut v = v.clone();
                b.iter(|| return unsafe { add_two_with_intrinsics_implicit(&mut v) });
            }
        });

    let functions = vec![
        // simd_inline,
        // simd_iter,
        // simd_iter_mut,
        // simd_par_iter_mut,
        // simd_par_iter,
        // simd_par_intoiter,
        // simd_branching_no_sorting,
        // simd_branching_no_sorting_par,
        simd_with_avx2_intrinsics_auto,
        simd_with_avx2_intrinsics_auto_par,
        simd_with_avx2_intrinsics_explicit,
    ];

    // let test_vector: Vec<i32> = vec![1; 100_000];
    use rand::distributions::Standard;
    let mut rng = rand::thread_rng();
    let test_vector: Vec<i32> = rng
        .sample_iter(&Standard)
        .take(1 << 16)
        .map(|x: i32| (x % 10) + 1)
        .collect();

    c.bench_functions("Simdings", functions, test_vector);
}

criterion_group!(benches, simd);
criterion_main!(benches);
