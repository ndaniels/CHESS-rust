#![feature(test)]

extern crate test;
use test::Bencher;
extern crate distance;
use distance::cosine;

#[bench]
fn big_euclidean(b: &mut Bencher) {
    b.iter(|| {
        let n = 100000000;
    let x: Vec<f64> = vec![0.2; n];
    let y: Vec<f64> = vec![0.3; n];
    cosine(&x,&y);
    
    });
}
