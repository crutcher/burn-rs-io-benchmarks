use serde::{Deserialize, Serialize};
use statrs::statistics::Statistics;
use std::time::Duration;

#[derive(Serialize, Deserialize, Debug)]
pub struct EventSamples {
    pub durations: Vec<Duration>,
}

impl EventSamples {
    pub fn new() -> Self {
        Self {
            durations: Vec::new(),
        }
    }

    pub fn push(
        &mut self,
        duration: Duration,
    ) {
        self.durations.push(duration);
    }

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

    pub fn sample_events<F>(
        reps: usize,
        mut f: F,
    ) -> Self
    where
        F: FnMut() -> Duration,
    {
        let mut samples = Self::new();

        for _ in 0..reps {
            samples.push(f());
        }

        samples
    }

    pub fn timing_stats(
        &self,
        outliers: usize,
    ) -> TimingStats {
        let times = &self.durations;
        let mut seconds: Vec<f64> = times.iter().map(|d| d.as_secs_f64()).collect();
        let duration_mean = seconds.clone().mean();

        // sort seconds by distance from mean.
        let mut seconds = {
            seconds.sort_by(|a, b| {
                let a = (a - duration_mean).abs();
                let b = (b - duration_mean).abs();
                a.partial_cmp(&b).unwrap()
            });
            seconds
        };

        // Drop outliers
        seconds.truncate(seconds.len() - outliers);
        let seconds = seconds;

        let samples = seconds.len();

        let duration_mean = seconds.clone().mean();
        let duration_std_dev = seconds.std_dev();

        let duration_mean = Duration::from_secs_f64(duration_mean);
        let duration_std_dev = Duration::from_secs_f64(duration_std_dev);

        TimingStats {
            mean: duration_mean,
            std_dev: duration_std_dev,
            samples,
            outliers,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TimingStats {
    pub mean: Duration,
    pub std_dev: Duration,

    pub samples: usize,
    pub outliers: usize,
}
