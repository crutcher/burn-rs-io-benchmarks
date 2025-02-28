use rand::distr::{Distribution, StandardUniform};
use rand::Rng;
use statrs::statistics::Statistics;

/// Generate a random vector of a given size.
pub fn random_vec<T>(size: usize) -> Vec<T>
where
    StandardUniform: Distribution<T>,
{
    let mut rng = rand::rng();
    (0..size).map(|_| rng.random()).collect()
}

/// Drop outliers (by distance from `.mean()`) from a list of durations.
///
/// # Arguments
/// * `data` - List of durations.
/// * `outliers` - Number of outliers to drop.
///
/// # Returns
/// A list of durations with the outliers removed.
pub fn drop_outliers(
    data: &Vec<f64>,
    outliers: usize,
) -> Vec<f64> {
    let mean = data.mean();

    let mut idxs: Vec<usize> = (0..data.len()).collect();
    idxs.sort_by(|a, b| {
        let a = (data[*a] - mean).abs();
        let b = (data[*b] - mean).abs();
        a.partial_cmp(&b).unwrap()
    });
    idxs.truncate(idxs.len() - outliers);
    idxs.sort();

    idxs.iter().map(|&i| data[i]).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_vec() {
        let vec = random_vec::<u8>(10);
        assert_eq!(vec.len(), 10);
    }

    #[test]
    fn test_drop_outliers() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let outliers = 2;

        let result = drop_outliers(&data, outliers);

        assert_eq!(result, vec![2.0, 3.0, 4.0]);
    }
}
