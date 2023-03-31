mod blackboard;
mod common;
mod imitative;

use crate::blackboard::{blackboard_get_args, BlackboardResultsWriter};
use crate::imitative::{imitative_get_args, ImitativeResultsWriter};
use indicatif::ProgressBar;
use itertools::{iproduct, Itertools};
use rayon::prelude::*;
use std::{cmp::Ordering, env, sync::Mutex};

enum RunType {
    Blackboard,
    Imitative,
    Error,
}

#[derive(Debug)]
struct InvalidRunType;

impl std::error::Error for InvalidRunType {}
impl std::fmt::Display for InvalidRunType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "error: invalid run type")
    }
}

fn choose_type() -> RunType {
    let args: Vec<String> = env::args().collect();
    match args.len().cmp(&2) {
        Ordering::Less => {
            println!(
                "info: must provide either ('blackboard' | 'b') or ('imitative' | 'i') as run type"
            );
            RunType::Error
        }
        _ => {
            let t = args.get(1).unwrap();
            match t.to_lowercase().as_str() {
                "blackboard" | "b" => {
                    println!("info: got run type blackboard");
                    RunType::Blackboard
                }
                "imitative" | "i" => {
                    println!("info: got run type imitative");
                    RunType::Imitative
                }
                _ => {
                    println!("info: invalid run type {}", t);
                    RunType::Error
                }
            }
        }
    }
}

const CUTOFF_COMP_COST: f64 = 10.0;

fn run_blackboard() -> Result<(), Box<dyn std::error::Error>> {
    let config = blackboard_get_args()?;
    config.log();

    let group_sizes = config.group_sizes.unwrap();
    let blackboard_sizes = config.blackboard_sizes.unwrap();
    let compute_phi = config.compute_phi;

    let jobs_len = group_sizes.len() * blackboard_sizes.len() * config.n_repeat as usize;

    if config.use_threads {
        let file = Mutex::new(BlackboardResultsWriter::new(
            config.output.clone(),
            config.flush_frequency,
            compute_phi,
        ));

        let pb = Mutex::new(ProgressBar::new(jobs_len as u64));

        iproduct!(group_sizes, blackboard_sizes, 0..config.n_repeat)
            .collect_vec()
            .into_par_iter()
            .for_each(|(m, b, _)| {
                let r = blackboard::blackboard(m, b, CUTOFF_COMP_COST, compute_phi);
                pb.lock().unwrap().inc(1);
                file.lock().unwrap().write(r);
            });
    } else {
        let mut file = BlackboardResultsWriter::new(
            config.output.clone(),
            config.flush_frequency,
            compute_phi,
        );

        let pb = ProgressBar::new(jobs_len as u64);
        for (m, b, _) in iproduct!(group_sizes, blackboard_sizes, 0..config.n_repeat) {
            let r = blackboard::blackboard(m, b, CUTOFF_COMP_COST, compute_phi);
            pb.inc(1);
            file.write(r);
        }
    }
    Ok(())
}

fn run_imitative() -> Result<(), Box<dyn std::error::Error>> {
    let config = imitative_get_args()?;
    config.log();

    let group_sizes = config.group_sizes.unwrap();
    let p_values = config.p_values;

    let jobs_len = group_sizes.len() * p_values.len() * config.n_repeat as usize;

    if config.use_threads {
        let file = Mutex::new(ImitativeResultsWriter::new(
            config.output.clone(),
            config.flush_frequency,
        ));

        let pb = Mutex::new(ProgressBar::new(jobs_len as u64));

        iproduct!(group_sizes, p_values, 0..config.n_repeat)
            .collect_vec()
            .into_par_iter()
            .for_each(|(m, p, _)| {
                let r = imitative::imitative(m, p, CUTOFF_COMP_COST);
                pb.lock().unwrap().inc(1);
                file.lock().unwrap().write(r);
            });
    } else {
        let mut file = ImitativeResultsWriter::new(config.output.clone(), config.flush_frequency);

        let pb = ProgressBar::new(jobs_len as u64);
        for (m, p, _) in iproduct!(group_sizes, p_values, 0..config.n_repeat) {
            let r = imitative::imitative(m, p, CUTOFF_COMP_COST);
            pb.inc(1);
            file.write(r);
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    match choose_type() {
        RunType::Blackboard => run_blackboard(),
        RunType::Imitative => run_imitative(),
        RunType::Error => Err(InvalidRunType {})?,
    }
}
