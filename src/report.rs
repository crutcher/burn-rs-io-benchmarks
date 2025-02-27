use crate::bandwidth_stats::{BandwidthStats, BandwidthTrials};
use crate::event_samples::EventSamples;
use crate::rand_util;
use burn::prelude::{Backend, Int, Tensor};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Serialize, Deserialize, Debug)]
pub struct BackendIoReport {
    pub backend: String,
    pub device: String,

    pub to_backend: BandwidthTrials,
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

fn warm_backend<B: Backend>(device: &B::Device) {
    let tensor = Tensor::<B, 1>::from_data([0.0; 10], device);
    let _ = tensor.into_data();
}

impl BackendIoReport {
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
