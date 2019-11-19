mod linalg {
	extern crate rayon;
	use rayon::prelude::*;
	pub fn dot(x: &Vec<f64>, y: &Vec<f64>) -> f64 {
		let res: f64 = x.par_iter().zip(y.par_iter()).map(|(a, b)| a * b).sum();
		return res;
	}

	pub fn sub(x: &Vec<f64>, y: &Vec<f64>) -> Vec<f64> {
		let res: Vec<f64> = x.par_iter().zip(y.par_iter()).map(|(a, b)| a - b).collect();
		return res;
	}

}

mod distance {
	extern crate rayon;
	use crate::linalg::dot;
	use rayon::prelude::*;
	pub fn euclidean(x: &Vec<f64>, y: &Vec<f64>) -> f64 {
		let res: f64 = x.par_iter()
						.zip(y.par_iter())
						.map(|(a,b)| (a-b)*(a-b))
						.sum();
		return res.sqrt(); // ugly; why does typechecker hate
						   // putting sqrt() call chained to sum()?
	}

	pub fn cosine(x: &Vec<f64>, y: &Vec<f64>) -> f64 {
		let num = dot(x,y);
		let dem = dot(x,x).sqrt() * dot(y,y).sqrt();
		return num/dem;
	}

	pub fn manhattan(x: &Vec<i64>, y: &Vec<i64>) -> u64 {
		let res: u64 = x.par_iter()
						.zip(y.par_iter())
						.map(|(a,b)| i64::abs(a-b) as u64)
						.sum();
		return res;
	}

	pub fn hamming(x: &Vec<u64>, y: &Vec<u64>) -> u64 {
		let res = x.par_iter()
				   .zip(y.par_iter())
				   .map(|(a,b)| if a == b {0} else {1})
				   .sum();
		return res;
	}
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}