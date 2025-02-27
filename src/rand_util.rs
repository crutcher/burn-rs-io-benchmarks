use rand::distr::{Distribution, StandardUniform};
use rand::Rng;

pub fn random_vec<T>(size: usize) -> Vec<T>
where
    StandardUniform: Distribution<T>,
{
    let mut rng = rand::rng();
    (0..size).map(|_| rng.random()).collect()
}
