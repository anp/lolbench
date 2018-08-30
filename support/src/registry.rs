//! For working with the `Registry` struct from `marky_mark`. We should minimize
//! code bloat in marky_mark, as it's compiled into all benchmark binaries.

use super::Result;

use marky_mark::{Benchmark, Registry};

pub fn get_benches(runner: Option<&str>) -> Result<Vec<Benchmark>> {
    let (reg, _f) = Registry::from_disk()?;
    let benchmarks = reg.benches();

    info!("Found and parsed {} benchmarks.", benchmarks.len());

    Ok(if let Some(r) = runner {
        let b = benchmarks
            .into_iter()
            .filter(|b| b.runner.as_ref().map(String::as_str) == runner)
            .collect::<Vec<_>>();

        info!(
            "{} benchmarks assigned to the requested runner ({}).",
            b.len(),
            r
        );

        b
    } else {
        benchmarks
    })
}
