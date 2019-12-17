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
	pub fn mul<T: Num + Send + Sync + Copy + Sum>(x: &[T], y: &[T]) -> Vec<T> {
		x.par_iter()
					.zip(y.par_iter())
					.map(|(a, b)| (*a) * (*b))
					.collect()
	}

}

mod distance {
	extern crate rayon;
	use std::cmp::PartialEq;
	use crate::linalg::dot;
	use crate::linalg::sub;
	use crate::linalg::mul;
	use rayon::prelude::*;
	pub fn euclidean(x: &[f64], y: &[f64]) -> f64 {
		euclideansq(x,y).sqrt()
	}

	pub fn euclideansq(x: &[f64], y: &[f64]) -> f64 {
		let d = sub(x,y);
		mul(&d,&d).iter().sum()
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
	use crate::distance;
	use float_cmp::approx_eq;
    #[test]
    fn test_hamming() {
    	let x = [1, 2, 3];
    	let y = [1, 1, 1];
        assert_eq!(distance::hamming(&x,&y), 2);
    }
    #[test]
    fn test_manhattan() {
    	let x = [1, 2, 3];
    	let y = [1, 1, 1];
        assert_eq!(distance::manhattan(&x,&y), 3);
    }
    #[test]
    fn test_cosine() {
    	let x = [1.0, 1.0, 1.0];
    	let y = [1.0, 1.0, 1.0];
        assert!(approx_eq!(f64, distance::cosine(&x,&y), 1.0, ulps=2));
    }
    #[test]
    fn test_euclidean() {
    	let x = [0.0,3.0];
    	let y = [4.0,0.0];
        assert_eq!(distance::euclidean(&x,&y), 5.0);
    }
    #[test]
    fn test_euclideansq() {
    	let x = [0.0,3.0];
    	let y = [4.0,0.0];
        assert_eq!(distance::euclideansq(&x,&y), 25.0);
    }
}