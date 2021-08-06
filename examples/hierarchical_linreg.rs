use std::collections::HashMap;

use compute::prelude::{linspace, Distribution1D, Normal, Vector};
use talos::{
    samplers::{Gibbs, Sampler},
    utils::Data,
    *,
};
use talos_procs::model;

fn main() {
    // simulate and pack some data of mixed types
    let mut data = HashMap::new();

    let slopes = [2., 4.];
    let intercepts = [1., 5.];

    let n = 50;

    let x1 = linspace(0., 10., n);
    let y1 = slopes[0] * &x1 + intercepts[0] + Normal::new(0., 0.5).sample_n(n);

    let x2 = linspace(0., 10., n);
    let y2 = slopes[1] * &x2 + intercepts[1] + Normal::new(0., 0.5).sample_n(n);

    data.insert("x1", Data::FloatArray(x1));
    data.insert("y1", Data::FloatArray(y1));
    data.insert("x2", Data::FloatArray(x2));
    data.insert("y2", Data::FloatArray(y2));

    // 2 slopes + 2 intercepts + hierarchical slope + hierarchical intercept

    let s = Gibbs::new(&[0.05; 6]);

    let params = [0.5; 6];

    let samples = s
        .sample(lnlik, &params, &data, 10000)
        .into_iter()
        .map(|x| Vector::from(x))
        .collect::<Vec<_>>();

    for (i, s) in samples.into_iter().enumerate() {
        println!("d{} = {}", i, s);
    }
}

#[model("f64")]
fn lnlik(params: &[f64], data: &HashMap<&str, Data>) {
    let (ih, i1, i2, sh, s1, s2) = unpack_tuple!(params, 6);
    let x1 = unpack!(data["x1"], FloatArray);
    let y1 = unpack!(data["y1"], FloatArray);
    let x2 = unpack!(data["x2"], FloatArray);
    let y2 = unpack!(data["y2"], FloatArray);

    for i in 0..x1.len() {
        normal!(y1[i]; x1[i] * s1 + i1, 0.5_f64);
        normal!(y2[i]; x2[i] * s2 + i2, 0.5_f64);
    }

    normal!(i1; ih, 2_f64);
    normal!(i2; ih, 2_f64);

    normal!(s1; sh, 2_f64);
    normal!(s2; sh, 2_f64);

    normal!(ih; 5_f64, 3_f64);
    normal!(sh; 2_f64, 2_f64);
}
