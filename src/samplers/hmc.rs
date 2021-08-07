use super::Sampler;
use rayon::prelude::*;
use reverse::*;

#[derive(Debug, Clone)]
pub struct HMC {}

impl<'a> Sampler<Var<'a>> for HMC {
    fn step<F, S>(&self, f: F, params: &[Var<'a>], data: S) -> Vec<f64>
    where
        F: Fn(&[Var<'a>], S) -> Var<'a> + Copy + Send + Sync,
        S: Send + Sync + Copy,
    {
        todo!()
    }

    fn sample<F, S>(&self, f: F, inits: &[Var<'a>], data: S, n_samples: usize) -> Vec<Vec<f64>>
    where
        F: Fn(&[Var<'a>], S) -> Var<'a> + Copy + Send + Sync,
        S: Send + Sync + Copy,
    {
        todo!()
    }

    fn sample_par<F, S>(
        &self,
        f: F,
        inits: &[Var<'a>],
        data: S,
        n_samples: usize,
        n_chains: usize,
    ) -> Vec<Vec<Vec<f64>>>
    where
        F: Fn(&[Var<'a>], S) -> Var<'a> + Copy + Send + Sync,
        Self: Clone,
        S: Send + Sync + Copy,
    {
        todo!()
    }
}
