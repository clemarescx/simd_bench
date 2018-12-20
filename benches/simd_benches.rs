use nalgebra::{Point2, Vector2};
use rand::Rng;
use rayon::prelude::*;

#[macro_use]
extern crate criterion;
use criterion::{Criterion, Fun};

type POS_TYPE = Point2<i32>;
type VEL_TYPE = Vector2<i32>;

pub fn pos_update_iter_return(pos: &[POS_TYPE], vel: &[VEL_TYPE]) -> Vec<POS_TYPE> {
    pos.iter().zip(vel.iter()).map(|(p, v)| p + v).collect()
}

pub fn pos_update_iter_mut_outparam(pos: &mut [POS_TYPE], vel: &[VEL_TYPE]) {
    pos.iter_mut().zip(vel.iter()).for_each(|(p, v)| *p += v);
}

pub fn pos_update_par_iter_mut_outparam(pos: &mut [POS_TYPE], vel: &[VEL_TYPE]) {
    assert_eq!(pos.len(), vel.len());
    pos.par_iter_mut()
        .enumerate()
        .for_each(|(i, p)| *p += vel[i]);
}

pub fn add_two_par_iter_return_inner(pos: &[POS_TYPE], vel: &[VEL_TYPE]) -> Vec<POS_TYPE> {
    assert_eq!(pos.len(), vel.len());
    pos.into_par_iter()
        .enumerate()
        .map(|(i, p)| p + vel[i])
        .collect()
}

pub fn add_two_else_add_three_return(v: &[POS_TYPE]) -> Vec<POS_TYPE> {
    v.into_iter()
        .map(|&pos| if pos.x <= 5 { pos * -1 } else { pos * 1 })
        .collect()
}

pub fn add_two_else_add_three_par_return(v: &[POS_TYPE]) -> Vec<POS_TYPE> {
    v.into_par_iter()
        .map(|&pos| if pos.x <= 5 { pos * -1 } else { pos * 1 })
        .collect()
}

// #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
// #[target_feature(enable = "avx2")]
// pub unsafe fn add_two_with_intrinsics_auto(v: &mut [i32]) {
//     add_two_iter_mut_outparam(v)
// }

// #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
// #[target_feature(enable = "avx2")]
// pub unsafe fn add_two_with_intrinsics_auto_par(v: &mut [i32]) {
//     add_two_par_iter_mut_outparam(v)
// }

// #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
// #[target_feature(enable = "avx2")]
// pub unsafe fn add_two_with_intrinsics_implicit(v: &mut [i32]) {
//     use std::arch::x86_64::{
//         _mm256_add_epi32, _mm256_maskstore_epi32, _mm256_set1_epi32, _mm256_set_epi32,
//     };
//     assert_eq!(v.len() % 8, 0);
//     let p_count = v.len() / 8;
//     for i in 0..p_count {
//         let offset = i * 8;
//         let p = _mm256_set_epi32(
//             v[offset],
//             v[offset + 1],
//             v[offset + 2],
//             v[offset + 3],
//             v[offset + 4],
//             v[offset + 5],
//             v[offset + 6],
//             v[offset + 7],
//         );
//         let inc = _mm256_set1_epi32(2);
//         let res = _mm256_add_epi32(p, inc);
//         _mm256_maskstore_epi32(
//             v.as_mut_ptr().add(offset) as *mut _,
//             _mm256_set1_epi32(0xffff),
//             res,
//         );
//     }
// }

// benchmarks

pub fn simd(c: &mut Criterion) {
    let simd_iter = Fun::new("simd_iter", |b, v: &Vec<(POS_TYPE, VEL_TYPE)>| {
        let (pos, vel): (Vec<_>, Vec<_>) = v.iter().cloned().unzip();
        b.iter(|| pos_update_iter_return(&pos, &vel))
    });

    let simd_iter_mut = Fun::new("simd_iter_mut", |b, v: &Vec<(POS_TYPE, VEL_TYPE)>| {
        let mut v = v.clone();
        let (mut pos, vel): (Vec<_>, Vec<_>) = v.iter().cloned().unzip();
        b.iter(|| pos_update_iter_mut_outparam(&mut pos, &vel))
    });

    let simd_par_iter_mut = Fun::new("simd_par_iter_mut", |b, v: &Vec<(POS_TYPE, VEL_TYPE)>| {
        let mut v = v.clone();
        let (mut pos, vel): (Vec<_>, Vec<_>) = v.iter().cloned().unzip();
        b.iter(|| pos_update_par_iter_mut_outparam(&mut pos, &vel))
    });
    // let simd_par_iter = Fun::new("simd_par_iter", |b, v: &Vec<POS_TYPE>| {
    //     b.iter(|| add_two_par_iter_return(v))
    // });

    // let simd_par_intoiter = Fun::new("simd_par_intoiter", |b, v: &Vec<POS_TYPE>| {
    //     b.iter(|| add_two_par_intoiter_return(v))
    // });

    // let simd_branching_no_sorting = Fun::new("With if-else checks", |b, v: &Vec<POS_TYPE>| {
    //     b.iter(|| add_two_else_add_three_return(v))
    // });

    // let simd_branching_no_sorting_par =
    //     Fun::new("With if-else checks - parallel", |b, v: &Vec<POS_TYPE>| {
    //         b.iter(|| add_two_else_add_three_par_return(v))
    //     });

    // let simd_with_avx2_intrinsics_auto =
    //     Fun::new("With avx2 intrinsics - auto", |b, v: &Vec<i32>| {
    //         if is_x86_feature_detected!("avx2") {
    //             let mut v = v.clone();
    //             b.iter(|| unsafe { add_two_with_intrinsics_auto(&mut v) });
    //         }
    //     });

    // let simd_with_avx2_intrinsics_auto_par =
    //     Fun::new("With avx2 intrinsics - auto parallel", |b, v: &Vec<i32>| {
    //         if is_x86_feature_detected!("avx2") {
    //             let mut v = v.clone();
    //             b.iter(|| unsafe { add_two_with_intrinsics_auto_par(&mut v) });
    //         }
    //     });

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
        // simd_with_avx2_intrinsics_auto,
        // simd_with_avx2_intrinsics_auto_par,
        // simd_with_avx2_intrinsics_explicit,
    ];

    // let test_vector: Vec<POS_TYPE> = vec![1; 100_000];
    // use rand::distributions::Standard;
    let mut rng = rand::thread_rng();
    const TEST_SIZE: usize = 1 << 16;
    let mut test_vector: Vec<(POS_TYPE, VEL_TYPE)> = Vec::with_capacity(TEST_SIZE);
    for (pos, vel) in test_vector.iter_mut() {
        *pos = Point2::new(rng.gen(), rng.gen());
        *vel = Vector2::new(rng.gen(), rng.gen());
    }

    // let test_vector: Vec<POS_TYPE> = rng
    //     .sample_iter(&Standard)
    //     .take(1 << 16)
    //     .map(|x: POS_TYPE| (x % 10) + 1)
    //     .collect();

    c.bench_functions("Simdings", functions, test_vector);
}

criterion_group!(benches, simd);
criterion_main!(benches);
