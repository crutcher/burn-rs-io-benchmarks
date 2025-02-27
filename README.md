# IO Benchmarks for Burn.Dev

Benchmarks for [Burn.Dev](https://burn.dev) Backend IO operations.

Features match Burn.dev names.

# Usage

```
$ cargo run --all-features --profile=release -- --help
...
Usage: burn-rs-io-benchmarks [OPTIONS] --backend <BACKEND>

Options:
      --backend <BACKEND>    Which backend to use [possible values: wgpu, cuda-jit]
      --min-pow <MIN_POW>    Minimum power of 2 for the size of the tensor [default: 4]
      --max-pow <MAX_POW>    Maximum power of 2 for the size of the tensor [default: 27]
      --reps <REPS>          Number of repetitions for each size (excluding outliers) [default: 20]
      --outliers <OUTLIERS>  Number of outliers to drop from the timing [default: 2]
      --output <OUTPUT>      Output mode [default: text] [possible values: json, text]
  -h, --help                 Print help
  -V, --version              Print version
````

# Example

```
$ cargo run --all-features --profile=release -- --backend=cuda-jit
...
Backend:
  burn_fusion::backend::Fusion<burn_jit::backend::JitBackend<cubecl_cuda::runtime::CudaRuntime, f32, i32, u8>>
Device:
  Cuda(0)

Cpu To Backend Trials:
    Payload |    Bandwidth |     Mean |  Std Dev |   N |   O
       16 B |    3.7 MiB/s |    4.1µs |    1.0µs |  20 |   2
       32 B |    7.8 MiB/s |    3.9µs |  928.0ns |  20 |   2
       64 B |   23.6 MiB/s |    2.6µs |  174.0ns |  20 |   2
      128 B |   49.8 MiB/s |    2.5µs |   41.0ns |  20 |   2
      256 B |   96.4 MiB/s |    2.5µs |   86.0ns |  20 |   2
      512 B |  180.7 MiB/s |    2.7µs |  487.0ns |  20 |   2
    1.0 KiB |  368.9 MiB/s |    2.6µs |  107.0ns |  20 |   2
    2.0 KiB |  649.7 MiB/s |    3.0µs |  100.0ns |  20 |   2
    4.0 KiB |    1.1 GiB/s |    3.4µs |   65.0ns |  20 |   2
    8.0 KiB |    1.8 GiB/s |    4.2µs |  142.0ns |  20 |   2
   16.0 KiB |    2.6 GiB/s |    5.8µs |  110.0ns |  20 |   2
   32.0 KiB |    3.4 GiB/s |    9.1µs |  435.0ns |  20 |   2
   64.0 KiB |    3.9 GiB/s |   15.7µs |  957.0ns |  20 |   2
  128.0 KiB |    4.9 GiB/s |   25.1µs |   97.0ns |  20 |   2
  256.0 KiB |    5.1 GiB/s |   48.3µs |    2.1µs |  20 |   2
  512.0 KiB |    5.2 GiB/s |   93.8µs |    4.6µs |  20 |   2
    1.0 MiB |    5.0 GiB/s |  195.1µs |   29.5µs |  20 |   2
    2.0 MiB |    5.3 GiB/s |  369.6µs |    7.9µs |  20 |   2
    4.0 MiB |    4.5 GiB/s |  877.6µs |  381.1µs |  20 |   2
    8.0 MiB |    4.9 GiB/s |    1.6ms |   94.5µs |  20 |   2
   16.0 MiB |    4.0 GiB/s |    3.9ms |   83.2µs |  20 |   2
   32.0 MiB |    1.2 GiB/s |   25.6ms |  192.9µs |  20 |   2
   64.0 MiB |    1.2 GiB/s |   51.9ms |  607.8µs |  20 |   2
  128.0 MiB |    1.2 GiB/s |  101.0ms |  821.5µs |  20 |   2

Backend To Cpu Trials:
    Payload |    Bandwidth |     Mean |  Std Dev |   N |   O
       16 B |    1.8 MiB/s |    8.4µs |  258.0ns |  20 |   2
       32 B |    3.6 MiB/s |    8.5µs |  199.0ns |  20 |   2
       64 B |    7.1 MiB/s |    8.6µs |  165.0ns |  20 |   2
      128 B |   14.2 MiB/s |    8.6µs |  236.0ns |  20 |   2
      256 B |   28.8 MiB/s |    8.5µs |  195.0ns |  20 |   2
      512 B |   56.8 MiB/s |    8.6µs |  213.0ns |  20 |   2
    1.0 KiB |  114.9 MiB/s |    8.5µs |  194.0ns |  20 |   2
    2.0 KiB |  218.9 MiB/s |    8.9µs |  209.0ns |  20 |   2
    4.0 KiB |  426.1 MiB/s |    9.2µs |  248.0ns |  20 |   2
    8.0 KiB |  805.7 MiB/s |    9.7µs |  138.0ns |  20 |   2
   16.0 KiB |    1.4 GiB/s |   11.0µs |  218.0ns |  20 |   2
   32.0 KiB |    2.2 GiB/s |   13.7µs |  385.0ns |  20 |   2
   64.0 KiB |    3.3 GiB/s |   18.6µs |  531.0ns |  20 |   2
  128.0 KiB |    4.2 GiB/s |   29.1µs |  778.0ns |  20 |   2
  256.0 KiB |    4.9 GiB/s |   49.8µs |    1.3µs |  20 |   2
  512.0 KiB |    6.0 GiB/s |   82.0µs |    1.2µs |  20 |   2
    1.0 MiB |    6.2 GiB/s |  156.3µs |    3.5µs |  20 |   2
    2.0 MiB |    7.5 GiB/s |  261.8µs |    4.0µs |  20 |   2
    4.0 MiB |    7.9 GiB/s |  495.1µs |    7.6µs |  20 |   2
    8.0 MiB |    7.1 GiB/s |    1.1ms |   29.0µs |  20 |   2
   16.0 MiB |  780.5 MiB/s |   20.5ms |   60.5µs |  20 |   2
   32.0 MiB |  760.1 MiB/s |   42.1ms |   66.0µs |  20 |   2
   64.0 MiB |  756.6 MiB/s |   84.6ms |  174.6µs |  20 |   2
  128.0 MiB |  757.6 MiB/s |  169.0ms |  639.2µs |  20 |   2
```