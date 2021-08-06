use reverse::*;

use compute::prelude::{Distribution, Normal};

pub struct Gibbs {
    stepsizes: Vec<f64>,
}

pub trait Sampler {
    type V;
    fn update_dim<'a, F, S>(&self, f: F, params: &[Self::V], data: &S, i: usize) -> Self::V
    where
        F: Fn(&[Self::V], &S) -> Self::V;
    fn update<'a, F, S>(&self, f: F, current_params: &[Self::V], data: &S) -> Vec<Self::V>
    where
        F: Fn(&[Self::V], &S) -> Self::V;
    fn sample<'a, F, S>(
        &self,
        f: F,
        inits: &[Self::V],
        data: &S,
        n_samples: usize,
    ) -> Vec<Vec<Self::V>>
    where
        F: Fn(&[Self::V], &S) -> Self::V;
}

impl Gibbs {
    pub fn new(stepsizes: &[f64]) -> Self {
        Self {
            stepsizes: stepsizes.to_vec(),
        }
    }

    #[inline]
    pub fn dims(&self) -> usize {
        self.stepsizes.len()
    }
}

impl Sampler for Gibbs {
    type V = f64;
    /// Make a proposal for the ith dimensional parameter.
    fn update_dim<'a, F, S>(&self, f: F, current_params: &[Self::V], data: &S, i: usize) -> Self::V
    where
        F: Fn(&[Self::V], &S) -> Self::V,
    {
        let mut proposed_params = current_params.to_vec();
        proposed_params[i] = proposed_params[i] + Normal::new(0., self.stepsizes[i]).sample();

        let current_likelihood = f(&current_params, data);
        let proposed_likelihood = f(&proposed_params, data);

        let p_accept = f64::min((proposed_likelihood - current_likelihood).exp(), 1.);

        if alea::f64() < p_accept {
            proposed_params[i]
        } else {
            current_params[i]
        }
    }

    /// Make a single proposal for the all parameters.
    fn update<'a, F, S>(&self, f: F, current_params: &[Self::V], data: &S) -> Vec<Self::V>
    where
        F: Fn(&[Self::V], &S) -> Self::V,
    {
        assert!(
            current_params.len() == self.dims(),
            "Wrong number of parameters."
        );

        let mut running_params = current_params.to_vec();

        for i in 0..self.dims() {
            running_params[i] = self.update_dim(&f, &running_params, data, i);
        }

        running_params
    }

    /// Get n_samples samples.
    fn sample<'a, F, S>(
        &self,
        f: F,
        inits: &[Self::V],
        data: &S,
        n_samples: usize,
    ) -> Vec<Vec<Self::V>>
    where
        F: Fn(&[Self::V], &S) -> Self::V,
    {
        assert!(inits.len() == self.dims(), "Wrong number of parameters.");

        let mut samples = Vec::with_capacity(n_samples);

        let mut running_params = inits.to_vec();

        for _ in 0..n_samples {
            running_params = self.update(&f, &running_params, data);
            samples.push(running_params.clone());
        }

        samples
    }
}
