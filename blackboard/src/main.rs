use blackboard::BlackboardResult;
use config::Config;
use indicatif::ProgressBar;
use itertools::{iproduct, Itertools};
use rayon::prelude::*;
use serde::Deserialize;
use std::{cmp::Ordering, env, fs::File, io::prelude::*, io::BufWriter, ops::Range, sync::Mutex};

#[derive(Debug, Deserialize, Clone)]
struct ConfigData {
    group_sizes: Option<Vec<u32>>,
    blackboard_sizes: Option<Vec<u32>>,
    group_range: Option<Range<u32>>,
    blackboard_range: Option<Range<u32>>,
    n_repeat: u32,
    output: String,
    compute_cost: bool,
    use_threads: bool,
    flush_frequency: u32,
}

impl ConfigData {
    // populates group_sizes and blackboard_sizes with Some
    fn standardize(&mut self) {
        let group_sizes = match &self.group_sizes {
            Some(v) => v.clone(),
            None => match &self.group_range {
                Some(r) => (r.start..=r.end).collect(),
                None => unreachable!(),
            },
        };
        let blackboard_sizes = match &self.blackboard_sizes {
            Some(v) => v.clone(),
            None => match &self.blackboard_range {
                Some(r) => (r.start..=r.end).collect(),
                None => unreachable!(),
            },
        };
        self.group_sizes = Some(group_sizes);
        self.blackboard_sizes = Some(blackboard_sizes);
    }
}

impl ConfigData {
    fn log(&self) {
        println!("info: starting batch",);
        println!("    - output:          {}", self.output);
        println!("    - use_threads:     {}", self.use_threads);
        println!("    - n_repeat:        {}", self.n_repeat);
        println!(
            "    - group:           {:?}",
            self.group_sizes.clone().unwrap()
        );
        println!(
            "    - blackboard:      {:?}",
            self.blackboard_sizes.clone().unwrap()
        );
        println!("    - compute_cost:    {}", self.compute_cost);
        println!("    - flush_frequency: {}", self.flush_frequency);
    }
}

fn get_args() -> Result<ConfigData, Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let config = match args.len().cmp(&2) {
        Ordering::Less => {
            println!(
                "info: no config file provided\ninfo: checking for config with name 'default'"
            );
            Config::builder()
                .add_source(config::File::with_name("default"))
                .set_default("flush_frequency", 100)?
                .set_default("use_threads", true)?
                .set_default("compute_cost", true)?
                .build()
        }
        Ordering::Greater => Err(config::ConfigError::Message(
            "provided more arguments than allowed".to_string(),
        )),
        _ => {
            let name = args.get(1).unwrap();
            println!("info: checking for config with name '{}'", name);
            Config::builder()
                .add_source(config::File::with_name(name))
                .set_default("flush_frequency", 100)?
                .set_default("use_threads", true)?
                .set_default("compute_cost", true)?
                .build()
        }
    }?;
    println!("info: loaded config file");
    let mut cfg = config.try_deserialize::<ConfigData>()?;
    if cfg.n_repeat == 0 {
        Err(config::ConfigError::Message(
            "n_repeat must be > 0".to_string(),
        ))?;
    }

    cfg.standardize();
    Ok(cfg)
}

fn computational_cost(m: u32, t: f64) -> f64 {
    (m as f64) * t / 3628800.0f64
}

struct ResultsWriter {
    file: BufWriter<File>,
    counter: u32,
    flush_frequency: u32,
    compute_cost: bool,
}

impl ResultsWriter {
    fn new(filename: String, compute_cost: bool, flush_frequency: u32) -> Self {
        Self {
            file: BufWriter::new(File::create(filename).unwrap()),
            counter: 0,
            flush_frequency,
            compute_cost,
        }
    }
    fn write(&mut self, result: Option<BlackboardResult>) -> bool {
        match result {
            Some(r) => {
                let _ = self.file.write(r.to_string().as_bytes());
                if self.compute_cost {
                    let _ = self
                        .file
                        .write(format!(",{}\n", computational_cost(r.m, r.t_star)).as_bytes());
                } else {
                    let _ = self.file.write(b"\n");
                }
                if self.counter == self.flush_frequency {
                    self.file.flush().unwrap();
                    self.counter = 0;
                }
                self.counter += 1;
                true
            }
            None => false,
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = get_args()?;
    config.log();

    let group_sizes = config.group_sizes.unwrap();
    let blackboard_sizes = config.blackboard_sizes.unwrap();

    let jobs_len = group_sizes.len() * blackboard_sizes.len() * config.n_repeat as usize;

    if config.use_threads {
        let file = Mutex::new(ResultsWriter::new(
            config.output.clone(),
            config.compute_cost,
            config.flush_frequency,
        ));

        let pb = Mutex::new(ProgressBar::new(jobs_len as u64));

        iproduct!(group_sizes, blackboard_sizes, 0..config.n_repeat)
            .collect_vec()
            .into_par_iter()
            .for_each(|(m, b, _)| {
                let r = blackboard::blackboard(m, b);
                pb.lock().unwrap().inc(1);
                file.lock().unwrap().write(r);
            });
    } else {
        let mut file = ResultsWriter::new(
            config.output.clone(),
            config.compute_cost,
            config.flush_frequency,
        );

        let pb = ProgressBar::new(jobs_len as u64);
        for (m, b, _) in iproduct!(group_sizes, blackboard_sizes, 0..config.n_repeat) {
            let r = blackboard::blackboard(m, b);
            pb.inc(1);
            file.write(r);
        }
    }

    Ok(())
}
