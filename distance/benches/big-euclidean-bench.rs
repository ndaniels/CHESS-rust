#![feature(test)]

extern crate test;
use test::Bencher;
extern crate distance;
use distance::euclidean;
use distance::levenshtein;

#[bench]
fn big_euclidean(b: &mut Bencher) {
    b.iter(|| {
        let n = 1000000;
	    let x: Vec<f64> = vec![0.2; n];
	    let y: Vec<f64> = vec![0.3; n];
	    euclidean(&x,&y);
    });
}

#[bench]
fn big_levenshtein(b: &mut Bencher) {
    b.iter(|| {
    let x = "abracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabraabracadabra";
    let y = "abracadabraabracadabraabracadabrbabracadabraabracadabraabracadabrbabracadabraabracadabraabracadabrbabracadabraabracadabraabracadabrbabracadabraabracadabraabracadabrbabracadabraabracadabraabracadabrbabracadabraabracadabraabracadabrbabracadabraabracadabraabracadabrbabracadabraabracadabraabracadabrbabracadabraabracadabraabracadabrbabracadabraabracadabraabracadabrbabracadabraabracadabraabracadabrbabracadabraabracadabraabracadabrbabracadabraabracadabraabracadabrbabracadabraabracadabraabracadabrbabracadabraabracadabraabracadabrbabracadabraabracadabraabracadabrbabracadabraabracadabraabracadabrbabracadabraabracadabraabracadabrbabracadabraabracadabraabracadabrbabracadabraabracadabraabracadabrbabracadabraabracadabraabracadabrbabracadabraabracadabraabracadabrbabracadabraabracadabraabracadabrbabracadabraabracadabraabracadabrbabracadabraabracadabraabracadabrbabracadabraabracadabraabracadabrbabracadabraabracadabraabracadabrbabracadabraabracadabraabracadabrbabracadabraabracadabraabracadabrbabracadabraabracadabraabracadabrbabracadabraabracadabraabracadabrbabracadabraabracadabraabracadabrbabracadabraabracadabraabracadabrbabracadabraabracadabraabracadabrbabracadabraabracadabraabracadabrbabracadabraabracadabraabracadabrbabracadabraabracadabraabracadabrbabracadabraabracadabraabracadabrbabracadabraabracadabraabracadabrbabracadabraabracadabraabracadabrb";
    levenshtein(&x,&y);
    });
}
