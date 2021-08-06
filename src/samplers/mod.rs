use rayon::prelude::*;

mod gibbs;
pub use gibbs::Gibbs;

pub trait Sampler {
    type V: Send + Sync + Copy;
    fn step<'a, F, S>(&self, f: F, params: &[Self::V], data: S) -> Vec<Self::V>
    where
        F: Fn(&[Self::V], S) -> Self::V + Copy + Send + Sync,
        S: Send + Sync + Copy;
    fn sample<'a, F, S>(
        &self,
        f: F,
        inits: &[Self::V],
        data: S,
        n_samples: usize,
    ) -> Vec<Vec<Self::V>>
    where
        F: Fn(&[Self::V], S) -> Self::V + Copy + Send + Sync,
        S: Send + Sync + Copy;
    fn sample_par<'a, F, S>(
        &self,
        f: F,
        inits: &[Self::V],
        data: S,
        n_samples: usize,
        n_chains: usize,
    ) -> Vec<Vec<Vec<Self::V>>>
    where
        F: Fn(&[Self::V], S) -> Self::V + Copy + Send + Sync,
        Self: Send + Sync,
        S: Send + Sync + Copy,
    {
        (0..n_chains)
            .into_par_iter()
            .map(|_| self.sample(f, inits, data, n_samples))
            .collect()
    }
}
