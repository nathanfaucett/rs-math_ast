#![feature(test)]


extern crate test;

extern crate math_ast;


use test::Bencher;

use math::*;


#[bench]
fn bench_math(b: &mut Bencher) {
    b.iter(|| {});
}
