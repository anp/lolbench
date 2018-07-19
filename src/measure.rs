use super::Result;

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use serde_json;

pub fn post_process(toolchain: &str) -> Result<()> {
    let mut measurements = BTreeMap::new();
    let target_dir = format!("target-{}", &toolchain);
    let criterion_dir = Path::new(&target_dir).join("criterion");

    println!("postprocessing");
    for entry in fs::read_dir(&criterion_dir).expect("reading criterion directory") {
        let entry = entry.expect("reading directory entry");
        let path = entry.path();
        let benchmark = path.file_name()
            .expect("finding the filename")
            .to_string_lossy()
            .to_string();

        if !entry.file_type().expect("finding the file type").is_dir() {
            continue;
        }

        let runtime_estimates_path = path.join("new").join("estimates.json");
        let metrics_estimates_path = path.join("new").join("metrics-estimates.json");

        let runtime_estimates_json = fs::read_to_string(runtime_estimates_path)?;
        let metrics_estimates_json = fs::read_to_string(metrics_estimates_path)?;

        let runtime_estimates: Estimates = serde_json::from_str(&runtime_estimates_json)?;
        let mut metrics_estimates: BTreeMap<String, Estimates> =
            serde_json::from_str(&metrics_estimates_json)?;

        metrics_estimates.insert(String::from("nanoseconds"), runtime_estimates);
        measurements.insert(benchmark, metrics_estimates);
    }

    let compiled = CompiledResults {
        toolchain: toolchain.to_string(),
        measurements,
    };

    fs::write(
        criterion_dir.join("lolbench-output.json"),
        serde_json::to_string_pretty(&compiled)?,
    )?;

    Ok(())
}

#[derive(Serialize)]
struct CompiledResults {
    toolchain: String,
    measurements: BTreeMap<String, BTreeMap<String, Estimates>>,
}

// the below is adapted from criterion

type Estimates = BTreeMap<Statistic, Estimate>;

#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Deserialize, Serialize, Debug)]
pub enum Statistic {
    Mean,
    Median,
    MedianAbsDev,
    Slope,
    StdDev,
}

#[derive(Clone, Copy, PartialEq, Deserialize, Serialize, Debug)]
struct ConfidenceInterval {
    confidence_level: f64,
    lower_bound: f64,
    upper_bound: f64,
}

#[derive(Clone, Copy, PartialEq, Deserialize, Serialize, Debug)]
struct Estimate {
    /// The confidence interval for this estimate
    confidence_interval: ConfidenceInterval,
    ///
    point_estimate: f64,
    /// The standard error of this estimate
    standard_error: f64,
}
