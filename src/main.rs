use std::collections::HashMap;

use compute::prelude::{linspace, Distribution1D, Normal, Vector};
use reverse::*;
use talos::samplers::{Gibbs, Sampler};
use talos::*;
use talos_procs::model;

fn main() {
    let s = Gibbs::new(&[0.1, 0.1]);

    let params = [0., 0.];

    let mut data = HashMap::new();
    let x = linspace(0., 10., 1000);
    let mut y = (&x) * 5. + 2.;
    y = y + Normal::new(0., 1.).sample_n(x.len());
    data.insert("x", x);
    data.insert("y", y);

    for samp in s.sample(lnlik, &params, &data, 10000) {
        println!("{},{}", samp[0], samp[1]);
    }
}

#[model("f64")]
fn lnlik(params: &[f64], data: &HashMap<&str, Vector>) {
    let (m, b) = (params[0], params[1]);

    normal!(m; 4_f64, 1_f64);
    laplace!(b; 2_f64, 1_f64);

    for (&xi, &yi) in data["x"].iter().zip(&data["y"]) {
        normal!(yi ; xi * m + b, 1.);
    }
}
