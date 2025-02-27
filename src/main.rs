use clap::{arg, Parser, ValueEnum};
use report::BackendIoReport;
use std::error::Error;

mod bandwidth_stats;
mod event_samples;
mod rand_util;
mod report;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct CliArgs {
    /// Which backend to use.
    #[arg(long, value_enum)]
    backend: BackendMode,

    /// Minimum power of 2 for the size of the tensor.
    #[arg(long, default_value_t = 4)]
    min_pow: usize,

    /// Maximum power of 2 for the size of the tensor.
    #[arg(long, default_value_t = 27)]
    max_pow: usize,

    /// Number of repetitions for each size (excluding outliers).
    #[arg(long, default_value_t = 20)]
    reps: usize,

    /// Number of outliers to drop from the timing.
    #[arg(long, default_value_t = 2)]
    outliers: usize,

    /// Output mode.
    #[arg(long, value_enum, default_value_t = OutputMode::Text)]
    output: OutputMode,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum BackendMode {
    #[cfg(feature = "wgpu")]
    Wgpu,

    #[cfg(feature = "cuda-jit")]
    CudaJit,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum OutputMode {
    Json,
    Text,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = CliArgs::parse();

    let sizes = (args.min_pow..args.max_pow + 1)
        .map(|k| 2usize.pow(k as u32))
        .collect::<Vec<_>>();

    let report = match args.backend {
        #[cfg(feature = "wgpu")]
        BackendMode::Wgpu => BackendIoReport::run::<burn::backend::Wgpu>(
            &Default::default(),
            sizes.as_slice(),
            args.reps,
            args.outliers,
        ),

        #[cfg(feature = "cuda-jit")]
        BackendMode::CudaJit => BackendIoReport::run::<burn::backend::CudaJit>(
            &Default::default(),
            sizes.as_slice(),
            args.reps,
            args.outliers,
        ),
    };

    match args.output {
        OutputMode::Json => {
            println!("{}", serde_json::to_string_pretty(&report).unwrap());
        }
        OutputMode::Text => {
            println!("{}", report);
        }
    }

    Ok(())
}
