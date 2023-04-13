#![allow(dead_code)]

mod blackboard;
mod common;
mod imitative;

pub use blackboard::{blackboard, blackboard_parallel, BlackboardResult};
pub use imitative::{imitative, imitative_parallel, ImitativeResult};
