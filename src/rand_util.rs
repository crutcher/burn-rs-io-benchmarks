use rand::distr::{Distribution, StandardUniform};
use rand::Rng;

/// Generate a random vector of a given size.
pub fn random_vec<T>(size: usize) -> Vec<T>
where
    StandardUniform: Distribution<T>,
{
    let mut rng = rand::rng();
    (0..size).map(|_| rng.random()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_vec() {
        let vec = random_vec::<u8>(10);
        assert_eq!(vec.len(), 10);
    }
}
