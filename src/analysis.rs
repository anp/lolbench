use super::*;

use std::collections::BTreeMap;

pub struct Analysis;

impl Analysis {
    pub fn from_estimates(_estimates: &BTreeMap<String, BTreeMap<Toolchain, Estimates>>) -> Self {
        Analysis
    }
}
