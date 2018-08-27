use std::collections::BTreeMap;
use std::time::{Duration, Instant};

use marky_mark::{Benchmark, Registry};
use noisy_float::prelude::*;

use super::Result;
use run_plan::RunPlan;
use CriterionConfig;

pub fn record_runtime_estimates() -> Result<()> {
    let (mut registry, _f) = Registry::from_disk()?;
    let benches = registry.benches();

    let updates_needed = benches.iter().any(|b| b.runtime_estimate.is_none());
    if !updates_needed {
        return Ok(());
    }

    let mut times = BTreeMap::new();

    for bench in registry.benches() {
        let key = bench.key();
        info!("timing {}", key);

        let plan = RunPlan::new(
            bench.clone(),
            Some(CriterionConfig {
                confidence_level: r32(0.95),
                measurement_time_ms: 1000, // FIXME do this better somehow
                noise_threshold: r32(0.0),
                nresamples: 10,
                sample_size: 10,
                significance_level: r32(0.05),
                warm_up_time_ms: 1,
            }),
            None,
            "stable".to_string(),
            bench.entrypoint_path.clone(),
            ::std::env::current_dir()?,
        )?;

        let start = Instant::now();

        let _results = plan.run()?;

        let total = Instant::now() - start;

        times.insert(key, total);
    }

    panic!("{:?}", times);

    let _ = registry.write()?;
    Ok(())
}
