use compute::prelude::{linspace, Distribution1D, Normal};
use talos::{
    samplers::{Gibbs, Sampler},
    *,
};
use talos_procs::model;

fn main() {
    // simulate some data
    let x = linspace(0., 10., 500);
    let mut y = (&x) * 1. + 2.;
    y = y + Normal::new(0., 0.1).sample_n(500);

    // make a RW Gibbs sampler with some stepsizes
    let s = Gibbs::new(&[0.2, 0.2, 0.1]);

    // guesses for parameters
    let inits = [4., 2., 1.];

    // sample with 4 parallel chains
    for chain in s.sample_par(lnlik, &inits, &[&x, &y], 10000, 4) {
        // remove burn-in and do thinning
        for samp in chain.iter().skip(2000).step_by(5) {
            println!("{}, {}, {}", samp[0], samp[1], samp[2].exp());
        }
    }
}

#[model("f64")]
fn lnlik(params: &[f64], data: &[&[f64]]) {
    let (m, b, s) = (params[0], params[1], params[2]);
    let x = &data[0];
    let y = &data[1];

    normal!(m; 4_f64, 2_f64);
    laplace!(b; 2_f64, 1_f64);
    exponential!(s; 1_f64);

    for i in 0..500 {
        normal!(y[i] ; x[i] * m + b, s.exp());
    }
}
