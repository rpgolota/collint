use crate::common::{computational_cost, Agent, Hint, Problem};
use config::Config;
use rand::seq::SliceRandom;
use serde::Deserialize;
use std::{cmp::Ordering, env, fs::File, io::prelude::*, io::BufWriter, ops::Range};

#[derive(Debug)]
pub struct BlackboardResult {
    pub m: u32,
    pub b: u32,
    pub t_star: f64,
    pub phi: f64,
    pub c: f64,
}

impl ToString for BlackboardResult {
    fn to_string(&self) -> String {
        format!("{},{},{},{}", self.m, self.b, self.t_star, self.c)
    }
}

fn calculate_phi(agents: &Vec<Agent>) -> f64 {
    let mut phi: f64 = 0.0;
    for a in agents.iter() {
        phi += (a.correct_hints as f64) / (a.total_hints as f64);
    }
    phi / (agents.len() as f64)
}

pub fn blackboard(m: u32, b: u32, max_c: f64, compute_phi: bool) -> Option<BlackboardResult> {
    let problem = Problem::new("DONALD", "GERALD", "ROBERT");

    let delta = 1.0 / (m as f64);
    let mut blackboard: Vec<Hint> = Vec::new();
    blackboard.reserve(b as usize);
    let mut agents = Vec::new();
    for _ in 0..m {
        agents.push(Agent::new(&problem, compute_phi));
    }

    let mut t = 1.0;

    for a in agents.iter_mut() {
        a.assign_random();
        a.find_hints();
        a.pick_and_replace(&mut blackboard);
    }

    while computational_cost(m, t) < max_c {
        let a = agents.choose_mut(&mut rand::thread_rng()).unwrap();
        a.make_move(&mut blackboard);
        a.find_hints();
        t += delta;
        a.pick_and_replace(&mut blackboard);
        if a.is_solved() {
            return Some(BlackboardResult {
                m,
                b,
                t_star: t,
                phi: calculate_phi(&agents),
                c: computational_cost(m, t),
            });
        }
    }
    None
}

#[derive(Debug, Deserialize, Clone)]
pub struct BlackboardConfigData {
    pub group_sizes: Option<Vec<u32>>,
    pub blackboard_sizes: Option<Vec<u32>>,
    pub group_range: Option<Range<u32>>,
    pub blackboard_range: Option<Range<u32>>,
    pub n_repeat: u32,
    pub output: String,
    pub use_threads: bool,
    pub flush_frequency: u32,
    pub compute_phi: bool,
}

impl BlackboardConfigData {
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

impl BlackboardConfigData {
    pub fn log(&self) {
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
        println!("    - flush_frequency: {}", self.flush_frequency);
        println!("    - compute_phi:     {}", self.compute_phi);
    }
}

pub fn blackboard_get_args() -> Result<BlackboardConfigData, Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let config = match args.len().cmp(&3) {
        Ordering::Less => {
            println!(
                "info: no config file provided\ninfo: checking for config with name 'blackboard_default'"
            );
            Config::builder()
                .add_source(config::File::with_name("blackboard_default"))
                .set_default("flush_frequency", 100)?
                .set_default("use_threads", true)?
                .set_default("compute_phi", false)?
                .build()
        }
        Ordering::Greater => Err(config::ConfigError::Message(
            "provided more arguments than allowed".to_string(),
        )),
        _ => {
            let name = args.get(2).unwrap();
            println!("info: checking for config with name '{}'", name);
            Config::builder()
                .add_source(config::File::with_name(name))
                .set_default("flush_frequency", 100)?
                .set_default("use_threads", true)?
                .set_default("compute_phi", false)?
                .build()
        }
    }?;
    println!("info: loaded config file");
    let mut cfg = config.try_deserialize::<BlackboardConfigData>()?;
    if cfg.n_repeat == 0 {
        Err(config::ConfigError::Message(
            "n_repeat must be > 0".to_string(),
        ))?;
    }

    cfg.standardize();
    Ok(cfg)
}

pub struct BlackboardResultsWriter {
    file: BufWriter<File>,
    counter: u32,
    flush_frequency: u32,
    compute_phi: bool,
}

impl BlackboardResultsWriter {
    pub fn new(filename: String, flush_frequency: u32, compute_phi: bool) -> Self {
        Self {
            file: BufWriter::new(File::create(filename).unwrap()),
            counter: 0,
            flush_frequency,
            compute_phi,
        }
    }
    pub fn write(&mut self, result: Option<BlackboardResult>) -> bool {
        match result {
            Some(r) => {
                let _ = self.file.write(r.to_string().as_bytes());
                if self.compute_phi {
                    let _ = self.file.write(format!(",{}\n", r.phi).as_bytes());
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
