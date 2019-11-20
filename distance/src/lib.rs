mod linalg {
	extern crate rayon;
	extern crate num;
	use num::Num;
	use std::iter::Sum;
	use rayon::prelude::*;
	pub fn dot<T: Num + Send + Sync + Copy + Sum>(x: &Vec<T>, y: &Vec<T>) -> T {
		let res:T  = x.par_iter().zip(y.par_iter()).map(|(a, b)| (*a) * (*b)).sum();
		return res;
	}

	pub fn sub<T: Num + Send + Sync + Copy + Sum>(x: &Vec<T>, y: &Vec<T>) -> Vec<T> {
		let res: Vec<T> = x.par_iter().zip(y.par_iter()).map(|(a, b)| (*a) - (*b)).collect();
		return res;
	}

}

mod distance {
	extern crate rayon;
	use std::cmp::PartialEq;
	use crate::linalg::dot;
	use rayon::prelude::*;
	pub fn euclidean(x: &Vec<f64>, y: &Vec<f64>) -> f64 {
		euclideansq(x,y).sqrt()
	}

	pub fn euclideansq(x: &Vec<f64>, y: &Vec<f64>) -> f64 {
		let res: f64 = x.par_iter()
						.zip(y.par_iter())
						.map(|(a,b)| (a-b)*(a-b))
						.sum();
		res
	}

	pub fn cosine(x: &Vec<f64>, y: &Vec<f64>) -> f64 {
		let num = dot(x,y);
		let dem = dot(x,x).sqrt() * dot(y,y).sqrt();
		num/dem
	}

	pub fn manhattan(x: &Vec<i64>, y: &Vec<i64>) -> u64 {
		let res = x.par_iter()
				   .zip(y.par_iter())
				   .map(|(a,b)| i64::abs(a-b) as u64)
				   .sum();
		res
	}

	pub fn hamming<T: PartialEq + Sync>(x: &Vec<T>, y: &Vec<T>) -> u64 {
		let res = x.par_iter()
				   .zip(y.par_iter())
				   .map(|(a,b)| if a == b {0} else {1})
				   .sum();
		res
	}
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}