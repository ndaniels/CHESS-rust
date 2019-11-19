mod linalg {
	extern crate rayon;
	use rayon::prelude::*;
	fn dot(x: Vec<f64>, y: Vec<f64>) -> f64 {
		let res: f64 = x.par_iter().zip(y.par_iter()).map(|(a, b)| a * b).sum();
		return res;
	}

	fn sub(x: Vec<f64>, y: Vec<f64>) -> Vec<f64> {
		let res: Vec<f64> = x.par_iter().zip(y.par_iter()).map(|(a, b)| a - b).collect();
		return res;
	}

}

mod distance {
	extern crate rayon;
	use rayon::prelude::*;
	fn euclidean(x: Vec<f64>, y: Vec<f64>) -> f64 {
		let res: f64 = x.par_iter().zip(y.par_iter()).map(|(a,b)| (a-b)*(a-b)).sum();
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