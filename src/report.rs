use crate::bandwidth_stats::{BandwidthStats, BandwidthTrials};
use crate::event_samples::EventSamples;
use crate::rand_util;
use burn::prelude::{Backend, Int, Tensor};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

/// Warm up the backend by creating a tensor and converting it back to data.
///
/// This ensures that lazily initialized device resources are created before the trials.
///
/// # Arguments
/// * `device` - The device to warm up.
fn warm_backend<B: Backend>(device: &B::Device) {
    let tensor = Tensor::<B, 1>::from_data([0.0; 10], device);
    let _ = tensor.into_data();
}

/// A report of I/O bandwidth trials.
#[derive(Serialize, Deserialize, Debug)]
pub struct BackendIoReport {
    /// Text description of the backend.
    pub backend: String,

    /// Text description of the device.
    pub device: String,

    /// Trials for sending data from the CPU to the backend.
    pub to_backend: BandwidthTrials,

    /// Trials for sending data from the backend to the CPU.
    pub from_backend: BandwidthTrials,
}

impl Display for BackendIoReport {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        writeln!(f, "Backend:\n  {}", self.backend)?;
        writeln!(f, "Device:\n  {}", self.device)?;

        fn format_trials(
            f: &mut std::fmt::Formatter<'_>,
            trials: &BandwidthTrials,
        ) -> std::fmt::Result {
            writeln!(
                f,
                " {:>10} | {:>12} | {:>8} | {:>8} | {:>3} | {:>3}",
                "Payload", "Bandwidth", "Mean", "Std Dev", "N", "O",
            )?;
            for trial in &trials.trials {
                writeln!(
                    f,
                    " {:>10} | {:>10}/s | {:>8.1?} | {:>8.1?} | {:>3} | {:>3}",
                    bytesize::ByteSize::b(trial.payload_size as u64),
                    bytesize::ByteSize::b(trial.bandwidth.bytes_per_second() as u64),
                    trial.timing_stats.mean,
                    trial.timing_stats.std_dev,
                    trial.timing_stats.samples,
                    trial.timing_stats.outliers,
                )?;
            }
            Ok(())
        }

        writeln!(f)?;
        writeln!(f, "Cpu To Backend Trials:")?;
        format_trials(f, &self.to_backend)?;

        writeln!(f)?;
        writeln!(f, "Backend To Cpu Trials:")?;
        format_trials(f, &self.from_backend)?;

        Ok(())
    }
}

/// Run trials for sending data from the CPU to the backend.
///
/// The total trials will be `reps + outliers` for each payload size;
/// and `outliers` furthest from the mean will be dropped.
///
/// # Arguments
/// * `device` - The device to use for the trials.
/// * `sizes` - The sizes of the payloads to test.
/// * `reps` - The number of repetitions for each size (excluding outliers).
/// * `outliers` - The number of outliers to drop from the timing.
///
/// # Returns
/// A `BandwidthTrials` object containing the trials.
fn run_cpu_to_backend_io_trials<B: Backend>(
    device: &B::Device,
    sizes: &[usize],
    reps: usize,
    outliers: usize,
) -> BandwidthTrials {
    let mut trials = Vec::new();

    for &payload_size in sizes {
        let len = payload_size / 4;
        assert_ne!(len, 0);

        // TODO: Consider testing the round-trip data for correctness.
        // If we do this, might as well time in both directions.

        let data: Vec<i32> = rand_util::random_vec(len);
        let slice = &data[..];

        trials.push(BandwidthStats::from_samples(
            payload_size,
            outliers,
            &EventSamples::time_events(reps + outliers, || {
                let _ = Tensor::<B, 1, Int>::from_data(slice, device);
            }),
        ));
    }

    BandwidthTrials { trials }
}

/// Run trials for sending data from the backend to the CPU.
///
/// The total trials will be `reps + outliers` for each payload size;
/// and `outliers` furthest from the mean will be dropped.
///
/// # Arguments
/// * `device` - The device to use for the trials.
/// * `sizes` - The sizes of the payloads to test.
/// * `reps` - The number of repetitions for each size (excluding outliers).
/// * `outliers` - The number of outliers to drop from the timing.
///
/// # Returns
/// A `BandwidthTrials` object containing the trials.
fn run_backend_to_cpu_io_trials<B: Backend>(
    device: &B::Device,
    sizes: &[usize],
    reps: usize,
    outliers: usize,
) -> BandwidthTrials {
    let mut trials = Vec::new();

    for &payload_size in sizes {
        let len = payload_size / 4;
        assert_ne!(len, 0);

        // TODO: Consider testing the round-trip data for correctness.
        // If we do this, might as well time in both directions.

        let data: Vec<i32> = rand_util::random_vec(len);
        let tensor = Tensor::<B, 1, Int>::from_data(&data[..], device);

        trials.push(BandwidthStats::from_samples(
            payload_size,
            outliers,
            &EventSamples::time_events(reps + outliers, || {
                let _ = tensor.to_data();
            }),
        ));
    }

    BandwidthTrials { trials }
}

impl BackendIoReport {
    /// Run the backend I/O trials.
    ///
    /// The total trials will be `reps + outliers` for each payload size;
    /// and `outliers` furthest from the mean will be dropped.
    ///
    /// # Arguments
    /// * `device` - The device to use for the trials.
    /// * `sizes` - The sizes of the payloads to test.
    /// * `reps` - The number of repetitions for each size (excluding outliers).
    /// * `outliers` - The number of outliers to drop from the timing.
    ///
    /// # Returns
    /// A `BackendIoReport` object containing the trials.
    pub fn run<B: Backend>(
        device: &B::Device,
        sizes: &[usize],
        reps: usize,
        outliers: usize,
    ) -> Self {
        warm_backend::<B>(device);

        let to_backend = run_cpu_to_backend_io_trials::<B>(device, sizes, reps, outliers);
        let from_backend = run_backend_to_cpu_io_trials::<B>(device, sizes, reps, outliers);

        BackendIoReport {
            backend: std::any::type_name::<B>().to_string(),
            device: format!("{:?}", device),

            to_backend,
            from_backend,
        }
    }
}
