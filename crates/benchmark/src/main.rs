use std::{
    path::PathBuf,
    process::Command,
    time::{self, Duration},
};

use std::{
    cmp,
    fs::{self},
    io::{self, Write},
    path::Path,
};

mod plot;

#[derive(Clone)]
pub struct Run {
    pub duration: Duration,
    pub id: u32,
}

impl Run {
    fn new(duration: Duration, num_classes: u32) -> Self {
        Self {
            duration,
            id: num_classes,
        }
    }

    fn to_point(&self) -> (u32, u128) {
        (self.id, self.duration.as_millis())
    }

    fn cmp_num(&self, other: &Self) -> cmp::Ordering {
        self.id.cmp(&other.id)
    }
}

pub struct BenchmarkResult {
    pub runs: Vec<Run>,
    is_sorted: bool,
}

impl BenchmarkResult {
    fn new() -> Self {
        Self {
            runs: Vec::new(),
            is_sorted: false,
        }
    }

    fn size(&self) -> usize {
        self.runs.len()
    }

    fn avg(&self) -> f64 {
        self.times().iter().map(|d| d.as_millis()).sum::<u128>() as f64 / self.runs.len() as f64
    }

    fn median(&self) -> f64 {
        let v: Vec<u128> = self.times().iter().map(|d| d.as_millis()).collect();
        let len = self.size();
        if len % 2 == 0 {
            v[len / 2] as f64
        } else {
            (v[len / 2] as f64 + v[len / 2 + 1] as f64) / 2.0
        }
    }

    fn push(&mut self, r: Run) {
        self.is_sorted = false;
        self.runs.push(r)
    }

    fn max_time(&self) -> Option<Duration> {
        self.runs.iter().map(|r| r.duration).max()
    }

    fn min_time(&self) -> Option<Duration> {
        self.runs.iter().map(|r| r.duration).min()
    }

    fn max_num(&self) -> Option<u32> {
        self.runs.iter().map(|r| r.id).max()
    }

    fn sort(&mut self) {
        if !self.is_sorted {
            self.runs.sort_by(|r1, r2| r1.cmp_num(r2));
            self.is_sorted = true;
        }
    }

    fn take(&mut self, n: usize) -> BenchmarkResult {
        self.sort();
        let runs = self
            .runs
            .iter()
            .take(n)
            .map(|r| r.clone())
            .collect::<Vec<Run>>();
        BenchmarkResult {
            runs,
            is_sorted: true,
        }
    }

    fn to_points(&mut self) -> Vec<(u32, u128)> {
        self.sort();
        self.runs.iter().map(Run::to_point).collect()
    }

    fn times(&self) -> Vec<Duration> {
        self.runs.iter().map(|r| r.duration).collect()
    }
}

fn get_num_classes(path: &PathBuf) -> u32 {
    let stem = path.file_stem().unwrap().to_str().unwrap();

    let mut num = String::new();
    for c in stem.chars() {
        if c.is_numeric() {
            num.push(c);
        }
    }
    num.parse().unwrap()
}

fn run_crowbar(path: PathBuf) -> Option<Run> {
    let num_classes = get_num_classes(&path);

    // Crowbar will fail anyway, safe us time
    if num_classes > 20 {
        return None;
    }

    let start = time::Instant::now();
    let mut cmd = Command::new("java");
    cmd.arg("-jar")
        .arg("../crowbar-tool/build/libs/crowbar-0.1-all.jar")
        .arg("--full")
        .arg(path);
    let output = cmd.output().expect("crowbar failed");
    //io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
    if !output.status.success() {
        return None;
    }
    Some(Run::new(start.elapsed(), num_classes))
}

fn run_nullable(path: PathBuf) -> Option<Run> {
    let num_classes = get_num_classes(&path);

    let start = time::Instant::now();
    let mut cmd = Command::new("../abstools/frontend/bin/bash/absc");
    cmd.arg(path);

    let output = cmd.output().expect("Nullable failed");
    //io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
    if !output.status.success() {
        return None;
    }
    Some(Run::new(start.elapsed(), num_classes))
}

fn main() -> io::Result<()> {
    let path = Path::new("./out/");

    let mut crowbar = BenchmarkResult::new();
    let mut nullable = BenchmarkResult::new();

    for e in fs::read_dir(path)? {
        let e = e?;
        let path = e.path();
        let name: String = path.file_name().unwrap().to_str().unwrap().to_string();
        println!("Current file: {}", name);
        if name.contains("cb") {
            if let Some(run) = run_crowbar(path) {
                crowbar.push(run)
            }
        } else {
            if let Some(run) = run_nullable(path) {
                nullable.push(run)
            }
        }
    }

    let mut nullable_first_20 = nullable.take(20);

    println!("\n=== Crowbar results ===");
    println!(
        "Number of runs: {}\nAverage time: {}\nMedian time: {}",
        crowbar.size(),
        crowbar.avg(),
        crowbar.median()
    );

    println!("\n=== Nullable results for the first 20 ===");
    println!(
        "Number of runs: {}\nAverage time: {}\nMedian time: {}",
        nullable_first_20.size(),
        nullable_first_20.avg(),
        nullable_first_20.median()
    );

    println!("\n=== Nullable results for all 100 ===");
    println!(
        "Number of runs: {}\nAverage time: {}\nMedian time: {}",
        nullable.size(),
        nullable.avg(),
        nullable.median()
    );

    plot::plot(&mut nullable_first_20, &mut crowbar).unwrap();
    plot::plot_nullable(&mut nullable).unwrap();

    Ok(())
}
