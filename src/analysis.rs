use super::*;

use std::collections::BTreeMap;

use itertools::Itertools;
use noisy_float::prelude::*;

#[derive(Clone, Debug, Serialize)]
pub struct Analysis {}

impl Analysis {
    pub fn from_estimates(
        _estimates: &BTreeMap<String, BTreeMap<Toolchain, (Vec<u8>, Estimates)>>,
    ) -> Self {
        Analysis {}
    }
}

pub fn geometric_mean(values: &[R64]) -> R64 {
    values
        .iter()
        .fold(r64(1.0), |a_n, &a_n1| a_n * a_n1)
        .powf(r64(1.0 / values.len() as f64))
}

pub fn normalized_against_first(
    values: impl IntoIterator<Item = R64>,
) -> impl IntoIterator<Item = R64> {
    let mut values = values.into_iter();
    let first = values.next().unwrap();
    ::std::iter::once(r64(1.0)).chain(values.map(move |v| v / first))
}

#[derive(Clone, Serialize)]
pub struct TimingRecord {
    pub binary_hash: String,
    pub toolchains: Vec<String>,
    pub anomaly_index: Option<AnomalyIndex>,
    pub metrics: RuntimeMetrics,
    pub normalized_metrics: RuntimeMetrics,
}

impl TimingRecord {
    pub fn new(
        current_binhash: &[u8],
        current_toolchains: &[Toolchain],
        current_measure: &Estimates,
        previous: &[Self],
    ) -> Self {
        let nice_hex =
            String::from_utf8(current_binhash.iter().fold(Vec::new(), |mut buf, byte| {
                use std::io::Write;
                buf.write_fmt(format_args!("{:x}", byte)).unwrap();
                buf
            })).unwrap();

        let metrics = RuntimeMetrics::from_measure(current_measure);
        let normalized_metrics = previous
            .get(0)
            .map(|first| metrics.normalized_against(&first.metrics))
            // this is only None if this is the first TimingRecord in the series, in which case
            // we'll normalize everything else as if this was a one
            .unwrap_or_else(RuntimeMetrics::ones);

        let mut timing = Self {
            binary_hash: nice_hex,
            toolchains: current_toolchains
                .into_iter()
                .map(|t| t.spec.clone())
                .sorted(),
            anomaly_index: None,
            metrics,
            normalized_metrics,
        };

        let anomaly_index = AnomalyIndex::new(&timing.metrics, previous.iter().map(|p| p.metrics));
        timing.anomaly_index = anomaly_index;
        timing
    }
}

#[derive(Copy, Clone, Serialize)]
pub struct RuntimeMetrics {
    pub nanoseconds: R64,
    pub instructions: R64,
    pub cpu_cycles: R64,

    pub context_switches: R64,
    pub cpu_clock: R64,

    pub branch_instructions: R64,
    pub branch_misses: R64,

    pub cache_misses: R64,
    pub cache_references: R64,
}

impl RuntimeMetrics {
    fn from_measure(current_measure: &Estimates) -> Self {
        let nanoseconds = r64(current_measure["nanoseconds"].median.point_estimate);
        let instructions = r64(current_measure["instructions"].median.point_estimate);
        let context_switches = r64(current_measure["context-switches"].median.point_estimate);
        let cpu_clock = r64(current_measure["cpu-clock"].median.point_estimate);
        let branch_instructions = r64(current_measure["branch-instructions"].median.point_estimate);
        let branch_misses = r64(current_measure["branch-misses"].median.point_estimate);
        let cache_misses = r64(current_measure["cache-misses"].median.point_estimate);
        let cache_references = r64(current_measure["cache-references"].median.point_estimate);
        let cpu_cycles = r64(current_measure["cpu-cycles"].median.point_estimate);

        RuntimeMetrics {
            nanoseconds,
            instructions,
            context_switches,
            cpu_clock,
            branch_instructions,
            branch_misses,
            cache_misses,
            cache_references,
            cpu_cycles,
        }
    }

    pub fn normalized_against(&self, baseline: &Self) -> Self {
        let n = |f: fn(&Self) -> R64| (f(self) + 1.0) / (f(baseline) + 1.0);
        Self {
            nanoseconds: n(|m| m.nanoseconds),
            instructions: n(|m| m.instructions),
            cpu_clock: n(|m| m.cpu_clock),
            branch_instructions: n(|m| m.branch_instructions),
            branch_misses: n(|m| m.branch_misses),
            cache_misses: n(|m| m.cache_misses),
            cache_references: n(|m| m.cache_references),
            cpu_cycles: n(|m| m.cpu_cycles),
            context_switches: n(|m| m.context_switches),
        }
    }

    pub fn ones() -> Self {
        let o = r64(1.0);
        RuntimeMetrics {
            nanoseconds: o,
            instructions: o,
            cpu_clock: o,
            branch_instructions: o,
            branch_misses: o,
            cache_misses: o,
            cache_references: o,
            cpu_cycles: o,
            context_switches: o,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct AnomalyIndex {
    pub nanoseconds: AnomalyScore,
    pub instructions: AnomalyScore,
    pub cpu_cycles: AnomalyScore,
    pub branch_instructions: AnomalyScore,
    pub branch_misses: AnomalyScore,
    pub cache_references: AnomalyScore,
    pub cache_misses: AnomalyScore,
}

impl AnomalyIndex {
    fn new(
        &RuntimeMetrics {
            nanoseconds,
            instructions,
            cpu_cycles,
            branch_instructions,
            branch_misses,
            cache_references,
            cache_misses,
            ..
        }: &RuntimeMetrics,
        previous: impl Clone + Iterator<Item = RuntimeMetrics>,
    ) -> Option<Self> {
        if let (
            Some(nanoseconds),
            Some(instructions),
            Some(cpu_cycles),
            Some(branch_instructions),
            Some(branch_misses),
            Some(cache_references),
            Some(cache_misses),
        ) = (
            AnomalyScore::new(nanoseconds, previous.clone().map(|p| p.nanoseconds)),
            AnomalyScore::new(instructions, previous.clone().map(|p| p.instructions)),
            AnomalyScore::new(cpu_cycles, previous.clone().map(|p| p.cpu_cycles)),
            AnomalyScore::new(
                branch_instructions,
                previous.clone().map(|p| p.branch_instructions),
            ),
            AnomalyScore::new(branch_misses, previous.clone().map(|p| p.branch_misses)),
            AnomalyScore::new(
                cache_references,
                previous.clone().map(|p| p.cache_references),
            ),
            AnomalyScore::new(cache_misses, previous.clone().map(|p| p.cache_misses)),
        ) {
            Some(Self {
                nanoseconds,
                instructions,
                cpu_cycles,
                branch_instructions,
                branch_misses,
                cache_references,
                cache_misses,
            })
        } else {
            None
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct AnomalyScore {
    pub kde_estimate: R64,
    pub percent_delta_from_mean: R64,
    pub stddev_from_mean: R64,
}

impl AnomalyScore {
    pub fn new(x: R64, previous: impl IntoIterator<Item = R64>) -> Option<Self> {
        use criterion_stats::univariate::{
            kde::{kernel::Gaussian, Bandwidth, Kde},
            Sample,
        };

        let x = x.raw();

        let (mut prev_contig, mut log_previous) = (vec![], vec![]);
        for prev in previous {
            prev_contig.push(prev.raw());
            log_previous.push(prev.ln_1p().raw());
        }

        if prev_contig.len() < NUM_SAMPLES {
            return None;
        }

        let sample = Sample::new(&prev_contig);

        let kde = Kde::new(Sample::new(&log_previous), Gaussian, Bandwidth::Silverman);
        let kde_estimate = R64::try_new(kde.estimate(x.ln_1p())).unwrap_or(r64(1000.0));

        let sample_mean = sample.mean();
        let percent_delta_from_mean =
            R64::try_new(((x - sample_mean) / sample_mean) * 100.0).unwrap_or(r64(0.0));
        let stddev_from_mean =
            R64::try_new((x - sample_mean) / sample.std_dev(Some(sample_mean))).unwrap_or(r64(0.0));

        Some(Self {
            kde_estimate,
            percent_delta_from_mean,
            stddev_from_mean,
        })
    }

    pub fn is_of_interest(&self) -> bool {
        self.kde_estimate < 10.0 && self.stddev_from_mean.abs().raw() > 2.0
    }
}

use std::fmt::{Display, Formatter, Result as FmtResult};

impl Display for AnomalyScore {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        use super::website::filters::float_fmt;
        f.write_fmt(format_args!(
            "score: {}, # of stddev from mean: {}, delta from sample mean: {} %",
            float_fmt(&self.kde_estimate.raw()).unwrap(),
            float_fmt(&self.stddev_from_mean.raw()).unwrap(),
            float_fmt(&self.percent_delta_from_mean.raw()).unwrap()
        ))
    }
}

impl<'a> ::std::ops::Index<&'a str> for AnomalyIndex {
    type Output = AnomalyScore;
    fn index(&self, i: &'a str) -> &Self::Output {
        match i {
            "nanoseconds" => &self.nanoseconds,
            "instructions" => &self.instructions,
            "cpu_cycles" => &self.cpu_cycles,
            "branch_instructions" => &self.branch_instructions,
            "branch_misses" => &self.branch_misses,
            "cache_references" => &self.cache_references,
            "cache_misses" => &self.cache_misses,
            _ => panic!(),
        }
    }
}

const NUM_SAMPLES: usize = 15;
