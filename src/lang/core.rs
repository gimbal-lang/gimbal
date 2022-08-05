use crate::lang::eval::{Exp};

fn plus_int(ints: Vec<i128>) -> i128 {
    ints.iter().sum()
}

fn plus_float(flts: Vec<f64>) -> f64 {
    flts.iter().sum()
}