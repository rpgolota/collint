use blackboard::BlackboardResult;
use config::{builder, Config};
use serde::Deserialize;
use std::{env, fs::File, io::prelude::*, io::BufWriter, ops::Range, vec};
use indicatif::{ProgressBar, ProgressStyle, ProgressState};

fn computational_cost(m: u32, t: f64) -> f64 {
    (m as f64) * t / 3628800.0f64
}

#[derive(Debug, Deserialize, Clone)]
struct ConfigData {
    group_sizes: Option<Vec<u32>>,
    blackboard_sizes: Option<Vec<u32>>,
    group_range: Option<Range<u32>>,
    blackboard_range: Option<Range<u32>>,
    n_repeat: u32,
    output: String,
    compute_cost: bool,
    n_threads: usize,
    flush_frequency: f64,
}

impl ConfigData {
    fn log(&self) {
        let exp = Experiment::from(self.clone());
        println!("info: starting batch",);
        println!("    - output:          {}", self.output);
        println!("    - n_threads:       {}", self.n_threads);
        println!("    - n_repeat:        {}", self.n_repeat);
        println!("    - group:           {:?}", exp.group_sizes);
        println!("    - blackboard:      {:?}", exp.blackboard_sizes);
        println!("    - compute_cost:    {}", self.compute_cost);
        println!("    - flush_frequency: {}", self.flush_frequency);
    }
}

#[derive(Debug)]
struct Experiment {
    group_sizes: Vec<u32>,
    blackboard_sizes: Vec<u32>,
    n_repeat: u32,
}

impl From<ConfigData> for Experiment {
    fn from(layout: ConfigData) -> Experiment {
        let group_sizes = match layout.group_sizes {
            Some(v) => v,
            None => match layout.group_range {
                Some(r) => (r.start..=r.end).collect(),
                None => vec![],
            },
        };
        let blackboard_sizes = match layout.blackboard_sizes {
            Some(v) => v,
            None => match layout.blackboard_range {
                Some(r) => (r.start..=r.end).collect(),
                None => vec![],
            },
        };

        Experiment {
            group_sizes: group_sizes,
            blackboard_sizes: blackboard_sizes,
            n_repeat: layout.n_repeat,
        }
    }
}

fn get_args() -> Result<ConfigData, Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let config = if args.len() < 2 {
        println!("info: no config file provided\ninfo: checking for config with name 'default'");
        Config::builder()
            .add_source(config::File::with_name("default"))
            .set_default("flush_frequency", 0.1)?
            .build()
    } else if args.len() > 2 {
        println!();
        Err(config::ConfigError::Message(
            "provided more arguments than allowed".to_string(),
        ))
    } else {
        let name = args.get(1).unwrap();
        println!("info: checking for config with name '{}'", name);
        Config::builder()
            .add_source(config::File::with_name(name))
            .set_default("flush_frequency", 0.1)?
            .build()
    }?;
    println!("info: loaded config file");
    let exp = config.try_deserialize::<ConfigData>()?;
    if exp.n_repeat == 0 {
        Err(config::ConfigError::Message(
            "n_repeat must be > 0".to_string(),
        ))?;
    }
    if exp.n_threads == 0 {
        Err(config::ConfigError::Message(
            "n_threads must be > 0".to_string(),
        ))?;
    }
    Ok(exp)
}

#[derive(Debug)]
struct Jobs {
    m: Vec<u32>,
    b: Vec<u32>,
    n: u32,
    mi: usize,
    bi: usize,
    ni: usize,
    finished: bool,
    max: u64
}

#[derive(Debug)]
struct Job {
    m: u32,
    b: u32,
}

impl Jobs {
    fn new(m: Vec<u32>, b: Vec<u32>, n: u32) -> Self {
        let max = (m.len() * b.len() * n as usize) as u64;
        Jobs {
            m: m,
            b: b,
            n: n,
            mi: 0,
            bi: 0,
            ni: 0,
            finished: false,
            max: max
        }
    }
    fn done(&self) -> bool {
        return self.finished;
    }
    fn next(&mut self) -> Job {
        let mi = self.mi;
        let bi = self.bi;
        self.ni += 1;
        if self.ni == self.n as usize {
            self.ni = 0;
            self.bi += 1;
            if self.bi == self.b.len() {
                self.bi = 0;
                self.mi += 1;
                if self.mi == self.m.len() {
                    self.mi = 0;
                    self.finished = true;
                }
            }
        }
        Job {
            m: *self.m.get(mi).unwrap(),
            b: *self.b.get(bi).unwrap(),
        }
    }
    fn total(&self) -> u64 {
        self.max
    }
}

struct ResultsWriter {
    file: BufWriter<File>,
    counter: u32,
    flush_period: u32,
    compute_cost: bool,
}

impl ResultsWriter {
    fn new(filename: String, compute_cost: bool, flush_period: u32) -> Self {
        Self {
            file: BufWriter::new(File::create(filename).unwrap()),
            counter: 0,
            flush_period: flush_period,
            compute_cost: compute_cost,
        }
    }
    fn write(&mut self, result: Option<BlackboardResult>) -> bool {
        match result {
            Some(r) => {
                self.file.write(r.to_string().as_bytes());
                if self.compute_cost {
                    self.file
                        .write(format!(",{}\n", computational_cost(r.m, r.t_star)).as_bytes());
                } else {
                    self.file.write(b"\n");
                }
                if self.counter == self.flush_period {
                    self.file.flush().unwrap();
                    self.counter = 0;
                }
                self.counter += 1;
                true
            }
            None => false
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = get_args()?;
    config.log();
    let experiment: Experiment = Experiment::from(config.clone());

    let mut experiment_results: ResultsWriter = ResultsWriter::new(
        config.output.clone(),
        config.compute_cost,
        ((experiment.group_sizes.len()
            * experiment.blackboard_sizes.len()
            * experiment.n_repeat as usize) as f64
            / config.flush_frequency) as u32,
    );

    let mut jobs = Jobs::new(
        experiment.group_sizes,
        experiment.blackboard_sizes,
        experiment.n_repeat,
    );

    let pb = ProgressBar::new(jobs.total());

    let mut handles: Vec<std::thread::JoinHandle<Option<BlackboardResult>>> =
        Vec::with_capacity(config.n_threads);

    for _ in 0..handles.capacity() {
        handles.push(std::thread::spawn(|| None));
    }

    // hand out all the jobs
    let mut n_finished: u64 = 0;
    while n_finished != jobs.total() {
        let mut to_join: Vec<usize> = vec![];
        for (i, h) in handles.iter().enumerate() {
            if h.is_finished() {
                to_join.push(i);
            }
        }
        for i in to_join {
            if experiment_results.write(handles.swap_remove(i).join().unwrap()) {
                pb.inc(1);
                n_finished += 1;
            }
            let job = jobs.next();
            handles.push(std::thread::spawn(move || {
                blackboard::blackboard(job.m, job.b)
            }));
        }
    }

    pb.finish();

    Ok(())
}
