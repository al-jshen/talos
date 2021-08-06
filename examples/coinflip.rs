use std::collections::HashMap;

use compute::prelude::{Binomial, Distribution1D, Vector};
use talos::{
    samplers::{Gibbs, Sampler},
    utils::Data,
    *,
};
use talos_procs::model;

fn main() {
    // simulate and pack some data of mixed types
    let mut data = HashMap::new();

    let trials = 100;

    let flips = Binomial::new(trials, 0.7).sample_n(50);

    data.insert(
        "heads",
        Data::IntArray(flips.iter().map(|&x| x as i32).collect()),
    );
    data.insert("trials", Data::Int(trials as i32));

    // run inference
    let s = Gibbs::new(&[0.01]);

    let params = [0.5];

    let samples = s
        .sample_par(lnlik, &params, &data, 10000, 4)
        .iter()
        .flatten()
        .map(|x| x[0])
        .collect::<Vector>();

    println!(
        "mean = {}, std = {}, true = 0.7",
        samples.mean(),
        samples.std()
    );
}

#[model("f64")]
fn lnlik(params: &[f64], data: &HashMap<&str, Data>) {
    let p = params[0];
    let heads = unpack!(data["heads"], IntArray);
    let trials = unpack!(data["trials"], Int);

    beta!(p; 2., 2.);

    for i in heads {
        binomial!(*i; *trials, p);
    }
}
