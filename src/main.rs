use std::collections::HashMap;

use compute::prelude::{
    linspace, Bernoulli, Binomial, Distribution, Distribution1D, Normal, Vector,
};
use reverse::*;
use talos::{
    samplers::{Gibbs, Sampler},
    *,
};
use talos_procs::model;

pub enum Data {
    Float(f64),
    Int(i32),
    FloatArray(Vector),
    IntArray(Vec<i32>),
}

fn main() {
    let s = Gibbs::new(&[0.01]);

    let params = [0.5];

    let mut data = HashMap::new();

    let trials = 100;

    let flips = Binomial::new(trials, 0.7).sample_n(50);

    data.insert(
        "heads",
        Data::IntArray(flips.iter().map(|&x| x as i32).collect()),
    );
    data.insert("trials", Data::Int(trials as i32));

    for samp in s.sample(lnlik, &params, &data, 1000) {
        // for samp in chain {
        println!("{}", samp[0]);
        // }
    }
}

#[model("f64")]
fn lnlik(params: &[f64], data: &HashMap<&str, Data>) {
    let p = params[0];
    let heads = match &data["heads"] {
        Data::IntArray(d) => d,
        _ => panic!(),
    };
    let trials = match data["trials"] {
        Data::Int(d) => d,
        _ => panic!(),
    };

    for i in heads {
        binomial!(*i; trials, p);
    }
}
