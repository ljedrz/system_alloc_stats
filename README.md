# system_alloc_stats
[![crates.io](https://img.shields.io/crates/v/system_alloc_stats)](https://crates.io/crates/system_alloc_stats)
[![docs.rs](https://docs.rs/system_alloc_stats/badge.svg)](https://docs.rs/system_alloc_stats)
[![actively maintained](https://img.shields.io/badge/Maintenance%20Level-Actively%20Maintained-green.svg)](https://gist.github.com/cheerfulstoic/d107229326a01ff0f333a1d3476e068d)

**system_alloc_stats** provides a wrapper around the [`System`](https://doc.rust-lang.org/std/alloc/struct.System.html) allocator, exposing some of its runtime statistics.

## Usage

```rust
use system_alloc_stats::SystemWithStats;

#[global_allocator]
static SWS: SystemWithStats = SystemWithStats;

fn main() {
    (...)
    println!("current heap use: {}; average allocation size: {}", SWS.use_curr(), SWS.alloc_avg());
    (...)
}
```