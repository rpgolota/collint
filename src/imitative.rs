use crate::common::{computational_cost, Agent, Problem};
use config::Config;
use rand::Rng;
use serde::Deserialize;
use std::{cmp::Ordering, env, fs::File, io::prelude::*, io::BufWriter, ops::Range};

#[derive(Debug)]
pub struct ImitativeResult {
    pub m: u32,
    pub p: f64,
    pub t_star: f64,
    pub c: f64,
}

impl ToString for ImitativeResult {
    fn to_string(&self) -> String {
        format!("{},{},{},{}\n", self.m, self.p, self.t_star, self.c)
    }
}

pub fn imitative(m: u32, p: f64, max_c: f64) -> Option<ImitativeResult> {
    let problem = Problem::new("DONALD", "GERALD", "ROBERT");

    let delta = 1.0 / (m as f64);
    let mut agents = Vec::new();
    for _ in 0..m {
        agents.push(Agent::new(&problem, false));
    }
    let mut best_agent: usize = 0;
    let mut best_cost: u32 = u32::MAX;

    let mut t = 1.0;

    for a in agents.iter_mut() {
        a.assign_random();
        a.compute_cost();
    }

    for (i, a) in agents.iter().enumerate() {
        if a.cost < best_cost {
            best_agent = i;
            best_cost = a.cost;
        }
    }

    while computational_cost(m, t) < max_c {
        let best = agents.get(best_agent).unwrap().clone();
        let i: usize = rand::thread_rng().gen_range(0..agents.len());
        let a = agents.get_mut(i).unwrap();

        if i != best_agent && rand::thread_rng().gen_bool(p) {
            a.imitate(&best);
        } else {
            a.elementary_move();
        }
        t += delta;
        a.compute_cost();
        if a.cost < best_cost {
            best_agent = i;
            best_cost = a.cost;
        }
        if a.cost == 0 {
            return Some(ImitativeResult {
                m,
                p,
                t_star: t,
                c: computational_cost(m, t),
            });
        }
    }
    None
}

#[derive(Debug, Deserialize, Clone)]
pub struct ImitativeConfigData {
    pub group_sizes: Option<Vec<u32>>,
    pub p_values: Vec<f64>,
    pub group_range: Option<Range<u32>>,
    pub n_repeat: u32,
    pub output: String,
    pub use_threads: bool,
    pub flush_frequency: u32,
}

impl ImitativeConfigData {
    // populates group_sizes and blackboard_sizes with Some
    fn standardize(&mut self) {
        let group_sizes = match &self.group_sizes {
            Some(v) => v.clone(),
            None => match &self.group_range {
                Some(r) => (r.start..=r.end).collect(),
                None => unreachable!(),
            },
        };
        self.group_sizes = Some(group_sizes);
    }
}

impl ImitativeConfigData {
    pub fn log(&self) {
        println!("info: starting batch",);
        println!("    - output:          {}", self.output);
        println!("    - use_threads:     {}", self.use_threads);
        println!("    - n_repeat:        {}", self.n_repeat);
        println!(
            "    - group:           {:?}",
            self.group_sizes.clone().unwrap()
        );
        println!("    - p:               {:?}", self.p_values.clone());
        println!("    - flush_frequency: {}", self.flush_frequency);
    }
}

pub fn imitative_get_args() -> Result<ImitativeConfigData, Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let config = match args.len().cmp(&3) {
        Ordering::Less => {
            println!(
                "info: no config file provided\ninfo: checking for config with name 'imitative_default'"
            );
            Config::builder()
                .add_source(config::File::with_name("imitative_default"))
                .set_default("flush_frequency", 100)?
                .set_default("use_threads", true)?
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
                .build()
        }
    }?;
    println!("info: loaded config file");
    let mut cfg = config.try_deserialize::<ImitativeConfigData>()?;
    if cfg.n_repeat == 0 {
        Err(config::ConfigError::Message(
            "n_repeat must be > 0".to_string(),
        ))?;
    }

    cfg.standardize();
    Ok(cfg)
}

pub struct ImitativeResultsWriter {
    file: BufWriter<File>,
    counter: u32,
    flush_frequency: u32,
}

impl ImitativeResultsWriter {
    pub fn new(filename: String, flush_frequency: u32) -> Self {
        Self {
            file: BufWriter::new(File::create(filename).unwrap()),
            counter: 0,
            flush_frequency,
        }
    }
    pub fn write(&mut self, result: Option<ImitativeResult>) -> bool {
        match result {
            Some(r) => {
                let _ = self.file.write(r.to_string().as_bytes());
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
