#![feature(test)]
extern crate rayon; 
use rayon::prelude::*;

extern crate test;

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn add_two_via_func(v: &mut Vec<i32>) {
   for i in 0..v.len() {
       v[i] = add_two(v[i]); 
   } 
}

pub fn add_two_via_lambda<T: Fn(i32)->i32>(v: &mut Vec<i32>, f: T){
   for i in 0..v.len() {
       v[i] = f(v[i]); 
   } 
}

pub fn add_two_inline(v: &mut Vec<i32>) {
   for i in 0..v.len() {
       v[i] += 2; 
   } 
}

pub fn add_two_iter(v: &mut Vec<i32>) {
   v.iter_mut().for_each( |x|  *x += 2);
}

pub fn add_two_par_iter_vec(v: &mut Vec<i32>) {
   v.par_iter_mut().for_each(|x| *x += 2)
}

pub fn add_two_par_iter_slice(v: &mut [i32]) {
   v.par_iter_mut().for_each(|x| *x += 2);
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
    
    #[test]
    fn it_works() {
        assert_eq!(4, add_two(2));
    }

    #[bench]
    fn bench_add_two(b: &mut Bencher) {
        b.iter(|| add_two(2));
    }

    #[bench]
    fn bench_simd_via_func(b: &mut Bencher) { 
        let mut v : Vec<i32> = vec![1; 100_000];
        b.iter(|| add_two_via_func(&mut v));
    }

    #[bench]
    fn bench_simd_via_lambda(b: &mut Bencher) { 
        let mut v : Vec<i32> = vec![1; 100_000];
        b.iter(|| add_two_via_lambda(&mut v, |x| x+2 ) );
    }
    
    #[bench]
    fn bench_simd_inline(b: &mut Bencher) { 
        let mut v : Vec<i32> = vec![1; 100_000];
        b.iter(|| add_two_inline(&mut v));
    }

    #[bench]
    fn bench_simd_iter(b: &mut Bencher) { 
        let mut v : Vec<i32> = vec![1; 100_000];
        b.iter(|| add_two_iter(&mut v));
    }

    #[bench]
    fn bench_simd_par_iter_vec(b: &mut Bencher) { 
        let mut v : Vec<i32> = vec![1; 100_000];
        b.iter(|| add_two_par_iter_vec(&mut v));
    }

    #[bench]
    fn bench_simd_par_iter_slice(b: &mut Bencher) { 
        let mut v : Vec<i32> = vec![1; 100_000];
        b.iter(|| add_two_par_iter_slice(&mut v));
    }
}