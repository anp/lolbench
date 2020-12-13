//! For working with the `Registry` struct from `marky_mark`. We should minimize
//! code bloat in marky_mark, as it's compiled into all benchmark binaries.

use super::Result;

use std::collections::BTreeMap;
use std::iter::FromIterator;
use std::path::Path;

use noisy_float::prelude::*;

use marky_mark::{Benchmark, Registry};

use storage::GitStore;

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

pub fn rebalance(sample_data_dir: impl AsRef<Path>) -> Result<()> {
    let (mut registry, _) = Registry::from_disk()?;
    let runners = registry.runners().to_owned();

    // fill the minheap with runners and 0 scores
    let mut weights = Vec::from_iter(runners.iter().map(|r| (r64(0.0), r.clone())));
    // also create a mapping of benchmark assignments
    let mut new_assignments: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let runners_by_bench_key = registry
        .benches()
        .into_iter()
        .filter_map(|b| {
            let bench_key = format!("{}::{}", b.crate_name, b.name);
            b.runner.map(|r| (bench_key, r))
        })
        .collect::<BTreeMap<String, String>>();

    for (ns, bench_key) in most_covered_toolchains_runtimes(sample_data_dir)? {
        if let Some(weight) = runners_by_bench_key
            .get(&bench_key)
            .and_then(|assigned| weights.iter_mut().find(|&&mut (_, ref r)| r == assigned))
        {
            // this has already been assigned, so we just need to track its runtime
            weight.0 += ns;
        } else {
            let &mut (ref mut current_score, ref runner) = &mut weights[0];

            *current_score += ns;
            new_assignments
                .entry(runner.to_owned())
                .or_default()
                .push(bench_key);
        }

        // this preserves this as a silly minheapish thing for next iteration
        weights.sort();
    }

    info!("new assignments: {:#?}", new_assignments);
    info!("weights: {:?}", weights);

    let all_benches = get_benches(None)?;
    let by_key = all_benches
        .into_iter()
        .map(|b| (b.key(), b))
        .collect::<BTreeMap<_, _>>();

    for (runner, bench_keys) in new_assignments {
        for bench_key in bench_keys {
            let mut benchmark: Benchmark = by_key[&bench_key].clone();
            benchmark.runner = Some(runner.to_owned());
            registry.update(&benchmark)?;
        }
    }

    Ok(())
}

fn most_covered_toolchains_runtimes(data_dir: impl AsRef<Path>) -> Result<Vec<(R64, String)>> {
    info!("finding all existing estimates");
    let estimates = GitStore::ensure_initialized(data_dir)?.all_stored_estimates()?;

    info!("reorganizing them by toolchain");
    let mut by_toolchain = BTreeMap::new();
    for (bench_key, toolchains) in estimates {
        for (toolchain, estimate) in toolchains {
            by_toolchain
                .entry(toolchain)
                .or_insert_with(BTreeMap::new)
                .insert(bench_key.clone(), estimate);
        }
    }

    let mut lens = by_toolchain
        .iter()
        .map(|(tc, by_bench)| (tc, by_bench.len()))
        .collect::<Vec<_>>();
    lens.sort();
    lens.reverse();
    info!(
        "the current toolchains and how many bench results they each have: {:#?}",
        lens
    );

    let (most_covered_toolchain, num_benches_covered) = lens[0];
    info!(
        "rebalancing based on toolchain '{:?}', it has {} benchmark results recorded",
        most_covered_toolchain, num_benches_covered
    );

    let most_covered = by_toolchain[&most_covered_toolchain].clone();

    let mut runtimes = most_covered
        .into_iter()
        .map(|(k, estimates)| (r64(estimates.1["nanoseconds"].median.point_estimate), k))
        .collect::<Vec<_>>();
    runtimes.sort();
    runtimes.reverse(); // we're going to binpack, starting with the largest

    Ok(runtimes)
}
