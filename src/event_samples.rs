use serde::{Deserialize, Serialize};
use statrs::statistics::Statistics;
use std::time::Duration;

/// Serializable list of event durations.
#[derive(Serialize, Deserialize, Debug)]
pub struct EventSamples {
    pub durations: Vec<Duration>,
}

/// Drop outliers (by distance from `.mean()`) from a list of durations.
///
/// # Arguments
/// * `data` - List of durations.
/// * `outliers` - Number of outliers to drop.
///
/// # Returns
/// A list of durations with the outliers removed.
fn drop_outliers(
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

impl EventSamples {
    /// Create a new list of event durations.
    pub fn new() -> Self {
        Self {
            durations: Vec::new(),
        }
    }

    /// Add a new event duration to the list.
    pub fn push(
        &mut self,
        duration: Duration,
    ) {
        self.durations.push(duration);
    }

    /// Time a function and collect the durations.
    ///
    /// # Arguments
    /// * `reps` - Number of repetitions.
    /// * `f` - Function to time.
    pub fn time_events<F>(
        reps: usize,
        mut f: F,
    ) -> Self
    where
        F: FnMut(),
    {
        EventSamples::sample_events(reps, || {
            let t_0 = std::time::Instant::now();

            f();

            let t_1 = std::time::Instant::now();

            t_1 - t_0
        })
    }

    /// Collect event durations.
    ///
    /// Use this function to collect event durations when
    /// the trials require internal setup and teardown outside
    /// the timed interval.
    ///
    /// # Arguments
    /// * `reps` - Number of repetitions.
    /// * `f` - Trial function, returns timing on each call.
    pub fn sample_events<F>(
        reps: usize,
        mut f: F,
    ) -> Self
    where
        F: FnMut() -> Duration,
    {
        let mut samples = Self::new();

        for _ in 0..reps {
            let d = f();
            samples.push(d);
        }

        samples
    }

    /// Calculate timing statistics for the event durations.
    ///
    /// # Arguments
    /// * `outliers` - Number of outliers to drop.
    pub fn timing_stats(
        &self,
        outliers: usize,
    ) -> TimingStats {
        let times = &self.durations;

        let seconds: Vec<f64> = times.iter().map(|d| d.as_secs_f64()).collect();
        let seconds = drop_outliers(&seconds, outliers);

        let samples = seconds.len();

        let mean = Duration::from_secs_f64(seconds.clone().mean());
        let std_dev = Duration::from_secs_f64(seconds.std_dev());

        TimingStats {
            mean,
            std_dev,
            samples,
            outliers,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TimingStats {
    /// Mean event duration.
    pub mean: Duration,

    /// Standard deviation of the event durations.
    pub std_dev: Duration,

    /// The number of samples used in the statistics.
    pub samples: usize,

    /// The number of outliers dropped from the statistics.
    pub outliers: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_drop_outliers() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let outliers = 2;

        let result = drop_outliers(&data, outliers);

        assert_eq!(result, vec![2.0, 3.0, 4.0]);
    }

    #[test]
    fn test_sample_events() {
        let mut fake_times = vec![
            Duration::from_secs(2),
            Duration::from_secs(3),
            Duration::from_secs(1),
        ];

        let samples = EventSamples::sample_events(3, || fake_times.pop().unwrap());

        assert_eq!(
            samples.durations,
            vec![
                Duration::from_secs(1),
                Duration::from_secs(3),
                Duration::from_secs(2),
            ]
        );
    }

    #[test]
    fn test_timing_stats() {
        let mut samples = EventSamples::new();
        samples.push(Duration::from_secs(1));
        samples.push(Duration::from_secs(2));
        samples.push(Duration::from_secs(3));
        samples.push(Duration::from_secs(4));
        samples.push(Duration::from_secs(5));

        let stats = samples.timing_stats(2);

        assert_eq!(stats.mean, Duration::from_secs(3));
        assert_eq!(stats.std_dev, Duration::from_secs(1));
        assert_eq!(stats.samples, 3);
        assert_eq!(stats.outliers, 2);
    }
}
