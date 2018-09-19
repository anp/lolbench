use super::*;

use std::collections::BTreeMap;

pub struct Analysis;

impl Analysis {
    pub fn from_estimates(_estimates: &BTreeMap<Toolchain, (Vec<u8>, Estimates)>) -> Self {
        Analysis
    }
}
