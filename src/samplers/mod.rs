mod gibbs;
mod hmc;
mod nuts;
pub use gibbs::Gibbs;
pub use hmc::HMC;
pub use nuts::NUTS;

pub trait Sampler<V> {
    fn step<'a, F, S>(&self, f: F, params: &[V], data: S) -> Vec<f64>
    where
        F: Fn(&[V], S) -> V + Copy + Send + Sync,
        S: Copy + Send + Sync;
    fn sample<'a, F, S>(&self, f: F, inits: &[V], data: S, n_samples: usize) -> Vec<Vec<f64>>
    where
        F: Fn(&[V], S) -> V + Copy + Send + Sync,
        S: Send + Sync + Copy;
    fn sample_par<'a, F, S>(
        &self,
        f: F,
        inits: &[V],
        data: S,
        n_samples: usize,
        n_chains: usize,
    ) -> Vec<Vec<Vec<f64>>>
    where
        F: Fn(&[V], S) -> V + Copy + Send + Sync,
        Self: Send + Sync + Clone,
        S: Send + Sync + Copy;
}
