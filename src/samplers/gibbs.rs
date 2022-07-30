use super::Sampler;
use compute::prelude::{Distribution, Normal};
use rayon::prelude::*;

#[derive(Debug, Clone)]
pub struct Gibbs {
    stepsizes: Vec<f64>,
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
    fn sample_par<'a, F, S>(
        &self,
        f: F,
        inits: &[f64],
        data: S,
        n_samples: usize,
        n_chains: usize,
    ) -> Vec<Vec<Vec<f64>>>
    where
        F: Fn(&[f64], S) -> f64 + Copy + Send + Sync,
        Self: Send + Sync + Clone,
        S: Send + Sync + Copy,
    {
        (0..n_chains)
            .into_par_iter()
            .map(|_| self.sample(f, inits, data, n_samples))
            .collect()
    }
}

impl Sampler<f64> for Gibbs {
    /// Make a single proposal for the all parameters.
    fn step<'a, F, S>(&self, f: F, current_params: &[f64], data: S) -> Vec<f64>
    where
        F: Fn(&[f64], S) -> f64 + Copy + Send + Sync,
        S: Copy + Send + Sync,
    {
        assert!(
            current_params.len() == self.dims(),
            "Wrong number of parameters."
        );

        let mut running_params = current_params.to_vec();

        for i in 0..self.dims() {
            running_params[i] = {
                let mut proposed_params = running_params.clone();
                proposed_params[i] =
                    proposed_params[i] + Normal::new(0., self.stepsizes[i]).sample();

                let current_likelihood = f(&running_params, data);
                let proposed_likelihood = f(&proposed_params, data);

                let p_accept = f64::min((proposed_likelihood - current_likelihood).exp(), 1.);

                if alea::f64() < p_accept {
                    proposed_params[i]
                } else {
                    running_params[i]
                }
            };
        }

        running_params
    }

    /// Get n_samples samples.
    fn sample<'a, F, S>(&self, f: F, inits: &[f64], data: S, n_samples: usize) -> Vec<Vec<f64>>
    where
        F: Fn(&[f64], S) -> f64 + Copy + Send + Sync,
        S: Copy + Send + Sync,
    {
        assert!(inits.len() == self.dims(), "Wrong number of parameters.");

        let mut samples = Vec::with_capacity(n_samples);

        let mut running_params = inits.to_vec();

        for _ in 0..n_samples {
            running_params = self.step(&f, &running_params, data);
            samples.push(running_params.clone());
        }

        (0..self.dims())
            .map(|i| samples.iter().map(|x| x[i]).collect::<Vec<_>>())
            .collect::<Vec<_>>()
    }
}
