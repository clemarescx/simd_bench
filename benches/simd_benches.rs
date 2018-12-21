use nalgebra::{Point2, Vector2};
use rand::Rng;
use rayon::prelude::ParallelIterator;

#[macro_use]
extern crate criterion;
use criterion::{Criterion, Fun};

use simd_bench::*;

// benchmarks

pub fn simd(c: &mut Criterion) {
    let simd_iter = Fun::new("simd_iter", |b, v: &Vec<(PosType, VelType)>| {
        let (pos, vel): (Vec<_>, Vec<_>) = v.iter().cloned().unzip();
        b.iter(|| pos_update_iter_return(&pos, &vel))
    });

    let simd_iter_mut = Fun::new("simd_iter_mut", |b, v: &Vec<(PosType, VelType)>| {
        // let v = v.clone();
        let (mut pos, vel): (Vec<_>, Vec<_>) = v.iter().cloned().unzip();
        b.iter(|| pos_update_iter_mut_outparam(&mut pos, &vel))
    });

    let simd_par_iter_mut = Fun::new("simd_par_iter_mut", |b, v: &Vec<(PosType, VelType)>| {
        // let v = v.clone();
        let (mut pos, vel): (Vec<_>, Vec<_>) = v.iter().cloned().unzip();
        b.iter(|| pos_update_par_iter_mut_outparam(&mut pos, &vel))
    });
    // let simd_par_iter = Fun::new("simd_par_iter", |b, v: &Vec<PosType>| {
    //     b.iter(|| add_two_par_iter_return(v))
    // });

    // let simd_par_intoiter = Fun::new("simd_par_intoiter", |b, v: &Vec<PosType>| {
    //     b.iter(|| add_two_par_intoiter_return(v))
    // });

    // let simd_branching_no_sorting = Fun::new("With if-else checks", |b, v: &Vec<PosType>| {
    //     b.iter(|| add_two_else_add_three_return(v))
    // });

    // let simd_branching_no_sorting_par =
    //     Fun::new("With if-else checks - parallel", |b, v: &Vec<PosType>| {
    //         b.iter(|| add_two_else_add_three_par_return(v))
    //     });

    let simd_with_avx2_intrinsics_auto = Fun::new(
        "With avx2 intrinsics - auto",
        |b, v: &Vec<(PosType, VelType)>| {
            if is_x86_feature_detected!("avx2") {
                let (mut pos, vel): (Vec<_>, Vec<_>) = v.iter().cloned().unzip();
                b.iter(|| unsafe { pos_update_with_intrinsics_auto(&mut pos, &vel) });
            }
        },
    );

    let simd_with_avx2_intrinsics_auto_par = Fun::new(
        "With avx2 intrinsics - auto parallel",
        |b, v: &Vec<(PosType, VelType)>| {
            if is_x86_feature_detected!("avx2") {
                let (mut pos, vel): (Vec<_>, Vec<_>) = v.iter().cloned().unzip();
                b.iter(|| unsafe { pos_update_with_intrinsics_auto_par(&mut pos, &vel) });
            }
        },
    );

    // let simd_with_avx2_intrinsics_explicit =
    //     Fun::new("With avx2 intrinsics - implicit", |b, v: &Vec<i32>| {
    //         if is_x86_feature_detected!("avx2") {
    //             let mut v = v.clone();
    //             b.iter(|| unsafe { add_two_with_intrinsics_implicit(&mut v) });
    //         }
    //     });

    let functions = vec![
        // simd_inline,
        simd_iter,
        simd_iter_mut,
        simd_par_iter_mut,
        // simd_par_iter,
        // simd_par_intoiter,
        // simd_branching_no_sorting,
        // simd_branching_no_sorting_par,
        simd_with_avx2_intrinsics_auto,
        simd_with_avx2_intrinsics_auto_par,
        // simd_with_avx2_intrinsics_explicit,
    ];

    const TEST_SIZE: usize = 1 << 16;

    let (posv, velv) = generate_data(TEST_SIZE);
    let test_vector: Vec<(PosType, VelType)> = posv.into_iter().zip(velv.into_iter()).collect();

    println!("test vector length: {}", test_vector.len());

    c.bench_functions("Simdings", functions, test_vector);
}

criterion_group!(benches, simd);
criterion_main!(benches);
