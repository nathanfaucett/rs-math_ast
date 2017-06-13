#![feature(test)]


extern crate test;

extern crate math_ast;


use test::Bencher;

use math_ast::from_str;


#[bench]
fn bench_math(b: &mut Bencher) {
    b.iter(|| {
        assert_eq!(from_str("|x + 1| / x").unwrap().to_tex(), "\\frac{|x + 1|}{x}")
    });
}
