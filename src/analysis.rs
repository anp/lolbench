use super::*;

use std::collections::BTreeMap;

use itertools::Itertools;
use noisy_float::prelude::*;

#[derive(Clone, Debug, Serialize)]
pub struct Analysis {
    pub anomalous_timings: Vec<(Toolchain, Vec<AnomalousTiming>)>,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct AnomalousTiming {
    pub index: AnomalyIndex,
    pub bench_fn: String,
    pub toolchain: Toolchain,
    pub timing: TimingRecord,
}

impl AnomalousTiming {
    pub fn benchmark_for_linking(&self) -> ::website::Benchmark {
        ::website::Benchmark::empty(self.bench_fn.clone())
    }
}

impl Analysis {
    pub fn new(timings: Vec<(String, TimingRecord)>) -> Self {
        let mut anomalous_timings = timings
            .iter()
            .filter(|(_, t)| {
                t.anomaly_index
                    .as_ref()
                    .map(|i| i.nanoseconds.is_of_interest())
                    .unwrap_or(false)
            }).fold(
                BTreeMap::<Toolchain, Vec<AnomalousTiming>>::new(),
                |mut anomalies, (bench_fn, timing)| {
                    let toolchain = timing.toolchains[0].clone();
                    {
                        let all_anomalies_for_toolchain =
                            anomalies.entry(toolchain.clone()).or_default();

                        all_anomalies_for_toolchain.push(AnomalousTiming {
                            bench_fn: bench_fn.clone(),
                            toolchain,
                            timing: timing.to_owned(),
                            index: timing.anomaly_index.unwrap(),
                        });

                        all_anomalies_for_toolchain.sort();
                    }
                    anomalies
                },
            ).into_iter()
            .collect::<Vec<_>>();

        // show the most recent toolchains first
        anomalous_timings.reverse();

        Analysis { anomalous_timings }
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

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct TimingRecord {
    pub binary_hash: String,
    pub toolchains: Vec<Toolchain>,
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

        let toolchains = current_toolchains.into_iter().cloned().sorted();

        let mut timing = Self {
            binary_hash: nice_hex,
            toolchains,
            anomaly_index: None,
            metrics,
            normalized_metrics,
        };

        let anomaly_index = AnomalyIndex::new(&timing.metrics, previous.iter().map(|p| p.metrics));
        timing.anomaly_index = anomaly_index;
        timing
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct MetricData {
    pub median: R64,
    pub lower_bound: R64,
    pub upper_bound: R64,
}

impl MetricData {
    pub fn median_only(median: R64) -> Self {
        MetricData {
            median,
            lower_bound: r64(0.0),
            upper_bound: r64(0.0),
        }
    }

    fn from_statistic(statistic: &Statistic) -> Self {
        MetricData {
            median: r64(statistic.median.point_estimate),
            lower_bound: r64(statistic.median.confidence_interval.lower_bound),
            upper_bound: r64(statistic.median.confidence_interval.upper_bound),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct RuntimeMetrics {
    pub nanoseconds: MetricData,
    pub instructions: MetricData,
    pub cpu_cycles: MetricData,

    pub context_switches: MetricData,
    pub cpu_clock: MetricData,

    pub branch_instructions: MetricData,
    pub branch_misses: MetricData,

    pub cache_misses: MetricData,
    pub cache_references: MetricData,
}

impl RuntimeMetrics {
    fn from_measure(current_measure: &Estimates) -> Self {
        let nanoseconds = MetricData::from_statistic(&current_measure["nanoseconds"]);
        let instructions = MetricData::from_statistic(&current_measure["instructions"]);
        let context_switches = MetricData::from_statistic(&current_measure["context-switches"]);
        let cpu_clock = MetricData::from_statistic(&current_measure["cpu-clock"]);
        let branch_instructions = MetricData::from_statistic(&current_measure["branch-instructions"]);
        let branch_misses = MetricData::from_statistic(&current_measure["branch-misses"]);
        let cache_misses = MetricData::from_statistic(&current_measure["cache-misses"]);
        let cache_references = MetricData::from_statistic(&current_measure["cache-references"]);
        let cpu_cycles = MetricData::from_statistic(&current_measure["cpu-cycles"]);

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
            nanoseconds: MetricData::median_only(n(|m| m.nanoseconds.median)),
            instructions: MetricData::median_only(n(|m| m.instructions.median)),
            cpu_clock: MetricData::median_only(n(|m| m.cpu_clock.median)),
            branch_instructions: MetricData::median_only(n(|m| m.branch_instructions.median)),
            branch_misses: MetricData::median_only(n(|m| m.branch_misses.median)),
            cache_misses: MetricData::median_only(n(|m| m.cache_misses.median)),
            cache_references: MetricData::median_only(n(|m| m.cache_references.median)),
            cpu_cycles: MetricData::median_only(n(|m| m.cpu_cycles.median)),
            context_switches: MetricData::median_only(n(|m| m.context_switches.median)),
        }
    }

    pub fn ones() -> Self {
        let o = r64(1.0);
        RuntimeMetrics {
            nanoseconds: MetricData::median_only(o),
            instructions: MetricData::median_only(o),
            cpu_clock: MetricData::median_only(o),
            branch_instructions: MetricData::median_only(o),
            branch_misses: MetricData::median_only(o),
            cache_misses: MetricData::median_only(o),
            cache_references: MetricData::median_only(o),
            cpu_cycles: MetricData::median_only(o),
            context_switches: MetricData::median_only(o),
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
            AnomalyScore::new(nanoseconds.median, previous.clone().map(|p| p.nanoseconds.median)),
            AnomalyScore::new(instructions.median, previous.clone().map(|p| p.instructions.median)),
            AnomalyScore::new(cpu_cycles.median, previous.clone().map(|p| p.cpu_cycles.median)),
            AnomalyScore::new(
                branch_instructions.median,
                previous.clone().map(|p| p.branch_instructions.median),
            ),
            AnomalyScore::new(branch_misses.median, previous.clone().map(|p| p.branch_misses.median)),
            AnomalyScore::new(
                cache_references.median,
                previous.clone().map(|p| p.cache_references.median),
            ),
            AnomalyScore::new(cache_misses.median, previous.clone().map(|p| p.cache_misses.median)),
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
            "delta from sample mean: {} %, {} stddev ",
            float_fmt(&self.percent_delta_from_mean.raw()).unwrap(),
            float_fmt(&self.stddev_from_mean.raw()).unwrap(),
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

const NUM_SAMPLES: usize = 10;
