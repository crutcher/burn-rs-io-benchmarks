use crate::event_samples::{EventSamples, TimingStats};
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Bandwidth description.
#[derive(Serialize, Deserialize, Debug)]
pub struct Bandwidth {
    /// Number of bytes transferred.
    pub bytes: u64,

    /// Duration of the transfer.
    pub duration: Duration,
}

impl Bandwidth {
    /// Create a new bandwidth description.
    ///
    /// # Arguments
    /// * `bytes` - Number of bytes transferred.
    /// * `duration` - Duration of the transfer.
    pub fn from_bytes(
        bytes: u64,
        duration: Duration,
    ) -> Self {
        Self { bytes, duration }
    }

    /// Calculate the bandwidth in bytes per second.
    pub fn bytes_per_second(&self) -> f64 {
        self.bytes as f64 / self.duration.as_secs_f64()
    }
}

/// Statistics for bandwidth trials.
#[derive(Serialize, Deserialize, Debug)]
pub struct BandwidthStats {
    /// Size of the payload in bytes.
    pub payload_size: usize,

    /// Timing statistics for the trials.
    pub timing_stats: TimingStats,

    /// Mean bandwidth.
    pub bandwidth: Bandwidth,
}

impl BandwidthStats {
    /// Create a new bandwidth statistics.
    ///
    /// # Arguments
    /// * `payload_size` - Size of the payload in bytes.
    /// * `timing_stats` - Timing statistics for the trials.
    pub fn new(
        payload_size: usize,
        timing_stats: TimingStats,
    ) -> BandwidthStats {
        let bandwidth = Bandwidth::from_bytes(payload_size as u64, timing_stats.mean);

        BandwidthStats {
            payload_size,
            timing_stats,
            bandwidth,
        }
    }

    /// Create bandwidth statistics from samples.
    ///
    /// # Arguments
    /// * `payload_size` - Size of the payload in bytes.
    /// * `outliers` - Number of outliers to remove.
    /// * `samples` - Event samples.
    pub fn from_samples(
        payload_size: usize,
        outliers: usize,
        samples: &EventSamples,
    ) -> Self {
        let timing_stats = samples.timing_stats(outliers);
        Self::new(payload_size, timing_stats)
    }
}

/// Bandwidth trials.
#[derive(Serialize, Deserialize, Debug)]
pub struct BandwidthTrials {
    pub trials: Vec<BandwidthStats>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bandwidth() {
        let bandwidth = Bandwidth::from_bytes(100, Duration::from_secs(1));
        assert_eq!(bandwidth.bytes_per_second(), 100.0);
    }

    #[test]
    fn test_bandwidth_stats_from_samples() {
        let mut samples = EventSamples::new();
        samples.push(Duration::from_secs(1));
        samples.push(Duration::from_secs(2));
        samples.push(Duration::from_secs(3));

        let bandwidth_stats = BandwidthStats::from_samples(100, 0, &samples);
        assert_eq!(bandwidth_stats.payload_size, 100);
        assert_eq!(bandwidth_stats.timing_stats.samples, 3);
        assert_eq!(bandwidth_stats.bandwidth.bytes_per_second(), 50.0);
    }
}
