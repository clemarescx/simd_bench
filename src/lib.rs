use nalgebra::{Point2, Vector2};
use rand::random;
use rayon::prelude::*;

pub type NumBaseType = i32;
pub type PosType = Point2<NumBaseType>;
pub type VelType = Vector2<NumBaseType>;

pub fn generate_data(entity_count: usize) -> (Vec<PosType>, Vec<VelType>) {
    (0..entity_count)
        .into_par_iter()
        .map(|_| {
            (
                Point2::new(random::<NumBaseType>() % 100, random::<NumBaseType>() % 100),
                Vector2::new(random::<NumBaseType>() % 100, random::<NumBaseType>() % 100),
            )
        })
        .unzip()
}

pub fn pos_update_iter_return(pos: &[PosType], vel: &[VelType]) -> Vec<PosType> {
    pos.iter().zip(vel.iter()).map(|(p, v)| p + v).collect()
}

pub fn pos_update_iter_mut_outparam(pos: &mut [PosType], vel: &[VelType]) {
    pos.iter_mut().zip(vel.iter()).for_each(|(p, v)| *p += v);
}

pub fn pos_update_par_iter_mut_outparam(pos: &mut [PosType], vel: &[VelType]) {
    pos.par_iter_mut()
        .enumerate()
        .for_each(|(i, p)| *p += vel[i]);
}

pub fn add_two_par_iter_return_inner(pos: &[PosType], vel: &[VelType]) -> Vec<PosType> {
    assert_eq!(pos.len(), vel.len());
    pos.into_par_iter()
        .enumerate()
        .map(|(i, p)| p + vel[i])
        .collect()
}

pub fn add_two_else_add_three_return(v: &[PosType]) -> Vec<PosType> {
    v.into_iter()
        .map(|&pos| if pos.x <= 5 { pos * -1 } else { pos })
        .collect()
}

pub fn add_two_else_add_three_par_return(v: &[PosType]) -> Vec<PosType> {
    v.into_par_iter()
        .map(|&pos| if pos.x <= 5 { pos * -1 } else { pos })
        .collect()
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[target_feature(enable = "avx2")]
pub unsafe fn pos_update_with_intrinsics_auto(pos: &mut [PosType], vel: &[VelType]) {
    pos_update_iter_mut_outparam(pos, vel);
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[target_feature(enable = "avx2")]
pub unsafe fn pos_update_with_intrinsics_auto_par(pos: &mut [PosType], vel: &[VelType]) {
    pos_update_par_iter_mut_outparam(pos, vel);
}

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
