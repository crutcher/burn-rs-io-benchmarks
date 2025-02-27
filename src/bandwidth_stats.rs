use crate::event_samples::{EventSamples, TimingStats};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::time::Duration;

#[derive(Serialize, Deserialize, Debug)]
pub struct Bandwidth {
    pub bytes: u64,
    pub duration: Duration,
}

impl Bandwidth {
    pub fn from_bytes(
        bytes: u64,
        duration: Duration,
    ) -> Self {
        Self { bytes, duration }
    }

    pub fn bytes_per_second(&self) -> f64 {
        self.bytes as f64 / self.duration.as_secs_f64()
    }
}

impl Display for Bandwidth {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        let display = bytesize::ByteSize::b(self.bytes_per_second() as u64);
        write!(f, "{}/s", display)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BandwidthStats {
    pub payload_size: usize,
    pub timing_stats: TimingStats,

    pub bandwidth: Bandwidth,
}

impl BandwidthStats {
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

    pub fn from_samples(
        payload_size: usize,
        outliers: usize,
        samples: &EventSamples,
    ) -> Self {
        let timing_stats = samples.timing_stats(outliers);
        Self::new(payload_size, timing_stats)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BandwidthTrials {
    pub trials: Vec<BandwidthStats>,
}
