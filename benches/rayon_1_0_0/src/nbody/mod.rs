pub mod bench;
mod nbody;
mod visualize;

#[derive(Copy, Clone, PartialEq, Eq, Deserialize)]
pub enum ExecutionMode {
    Par,
    ParReduce,
    Seq,
}
