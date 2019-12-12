mod linalg {
	extern crate rayon;
	extern crate num;
	use num::Num;
	use std::iter::Sum;
	use rayon::prelude::*;
	pub fn dot<T: Num + Send + Sync + Copy + Sum>(x: &[T], y: &[T]) -> T {
		x.par_iter()
					.zip(y.par_iter())
					.map(|(a, b)| (*a) * (*b))
					.sum()
	}

	pub fn sub<T: Num + Send + Sync + Copy + Sum>(x: &[T], y: &[T]) -> Vec<T> {
		x.par_iter()
					.zip(y.par_iter())
					.map(|(a, b)| (*a) - (*b))
					.collect()
	}

}

mod distance {
	extern crate rayon;
	use std::cmp::PartialEq;
	use crate::linalg::dot;
	use rayon::prelude::*;
	pub fn euclidean(x: &[f64], y: &[f64]) -> f64 {
		euclideansq(x,y).sqrt()
	}

	pub fn euclideansq(x: &[f64], y: &[f64]) -> f64 {
		x.par_iter()
					.zip(y.par_iter())
					.map(|(a,b)| (a-b)*(a-b))
					.sum()
	}

	pub fn cosine(x: &[f64], y: &[f64]) -> f64 {
		let num = dot(x,y);
		let dem = dot(x,x).sqrt() * dot(y,y).sqrt();
		num/dem
	}

	pub fn manhattan(x: &[i64], y: &[i64]) -> u64 {
		x.par_iter()
				   .zip(y.par_iter())
				   .map(|(a,b)| i64::abs(a-b) as u64)
				   .sum()
	}

	pub fn hamming<T: PartialEq + Sync>(x: &[T], y: &[T]) -> u64 {	
		x.par_iter()
				   .zip(y.par_iter())
				   .map(|(a,b)| if a == b {0} else {1})
				   .sum()
	}

}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}