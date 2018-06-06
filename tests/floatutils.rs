#![allow(dead_code)]

extern crate float_cmp;

use float_cmp::ApproxEqRatio;

pub fn assert_eq(a: f64, b: f64) {
    assert!(a.approx_eq_ratio(&b, 0.01), format!("{} == {}", a, b));
}

pub fn assert_ne(a: f64, b: f64) {
    assert!(!a.approx_eq_ratio(&b, 0.01), format!("{} != {}", a, b));
}
