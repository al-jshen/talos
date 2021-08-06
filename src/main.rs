use std::collections::HashMap;

use compute::prelude::{linspace, Distribution1D, Normal, Vector};
use reverse::*;
use talos::samplers::{Gibbs, Sampler};
use talos::*;
use talos_procs::model;

pub enum Data {
    Float(f64),
    Int(i32),
    FloatArray(Vector),
    IntArray(Vec<i32>),
}

fn main() {
    const n: usize = 2;

    let s = Gibbs::new(&[0.05; 2 + 2 * n]);

    let params = [0.; 2 + 2 * n];

    let mut data = HashMap::new();
    let mut x = linspace(0., 10., n);
    x = x + Normal::new(0., 1.).sample_n(n);
    let mut y = (&x) * 5. + 2.;
    y = y + Normal::new(0., 1.).sample_n(n);
    data.insert("xobs", Data::FloatArray(x));
    data.insert("yobs", Data::FloatArray(y));

    for chain in s.sample_par(lnlik, &params, &data, 10000, 4) {
        for samp in chain {
            println!("{},{}", samp[0], samp[1]);
        }
    }
}

#[model("f64")]
fn lnlik(params: &[f64], data: &HashMap<&str, Data>) {
    let (m, b) = (params[0], params[1]);
    let x = &params[2..4];
    let y = &params[4..6];
    let xobs = match &data["xobs"] {
        Data::FloatArray(arr) => arr,
        _ => panic!(),
    };
    let yobs = match &data["xobs"] {
        Data::FloatArray(arr) => arr,
        _ => panic!(),
    };

    normal!(m; 4_f64, 1_f64);
    laplace!(b; 2_f64, 1_f64);

    for i in 0..2 {
        normal!(xobs[i]; x[i], 1.);
        normal!(yobs[i]; x[i], 1.);
        normal!(y[i] ; x[i] * m + b, 1.);
    }
}
