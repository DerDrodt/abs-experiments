use std::{
    path::PathBuf,
    process::Command,
    time::{self, Duration},
};

use std::{
    cmp,
    fs::{self, File},
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

    fn format(&self) -> String {
        let mut out = String::new();
        for (i, r) in self.runs.iter().enumerate() {
            if i > 0 {
                out.push('\n');
            }
            out.push_str(&format!("{} {}", r.id, r.duration.as_millis()))
        }
        out
    }

    fn from_file(s: &str) -> Self {
        let mut res = Self::new();

        for line in s.split('\n') {
            let mut i = line.split(' ');
            let id = i.next().unwrap();
            let time = i.next().unwrap();
            let time: u128 = time.parse().unwrap();
            let d = Duration::from_millis(time as u64);
            let r = Run::new(d, id.parse().unwrap());
            res.push(r)
        }

        res
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

fn run_location(path: PathBuf, old: bool) -> Option<Run> {
    let num_classes = get_num_classes(&path);

    let start = time::Instant::now();
    let mut cmd = Command::new("../abstools/frontend/bin/bash/absc");
    cmd.arg("--loctypes").arg(path);

    if old {
        cmd.arg("--nonullablecheck");
    }

    let output = cmd.output().expect("Location failed");
    //io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
    if !output.status.success() {
        return None;
    }
    Some(Run::new(start.elapsed(), num_classes))
}

fn print_results(
    crowbar: &mut BenchmarkResult,
    nullable_20: &mut BenchmarkResult,
    nullable: &mut BenchmarkResult,
    location: &mut BenchmarkResult,
) {
    if crowbar.size() > 0 {
        println!("\n=== Crowbar results ===");
        println!(
            "Number of runs: {}\nAverage time: {}\nMedian time: {}",
            crowbar.size(),
            crowbar.avg(),
            crowbar.median()
        );
    }
    if nullable_20.size() > 0 {
        println!("\n=== Nullable results for the first 20 ===");
        println!(
            "Number of runs: {}\nAverage time: {}\nMedian time: {}",
            nullable_20.size(),
            nullable_20.avg(),
            nullable_20.median()
        );
    }
    if nullable.size() > 0 {
        println!("\n=== Nullable results for all 100 ===");
        println!(
            "Number of runs: {}\nAverage time: {}\nMedian time: {}",
            nullable.size(),
            nullable.avg(),
            nullable.median()
        );
    }
    if location.size() > 0 {
        println!("\n=== Location Type Inference results for all 100 ===");
        println!(
            "Number of runs: {}\nAverage time: {}\nMedian time: {}",
            location.size(),
            location.avg(),
            location.median()
        );
    }
}

fn main() -> io::Result<()> {
    let path = Path::new("./out/");

    let mut crowbar = BenchmarkResult::new();
    let mut nullable = BenchmarkResult::new();
    let mut location = BenchmarkResult::new();

    let loc = std::env::args().any(|a| a == "--loc");
    let plot_loc = std::env::args().any(|a| a == "--plot-loc");

    if plot_loc {
        use std::io::prelude::*;
        let mut new_f = File::open("loc-runs.txt")?;
        let mut new = String::new();
        new_f.read_to_string(&mut new)?;
        let mut old_f = File::open("loc-runs-old.txt")?;
        let mut old = String::new();
        old_f.read_to_string(&mut old)?;

        let mut new = BenchmarkResult::from_file(&new);
        let mut old = BenchmarkResult::from_file(&old);
        plot::plot_loc(&mut new, &mut old).unwrap()
    } else {
        for e in fs::read_dir(path)? {
            let e = e?;
            let path = e.path();
            let name: String = path.file_name().unwrap().to_str().unwrap().to_string();
            println!("Current file: {}", name);
            if loc {
                if name.contains("loc") {
                    if let Some(run) = run_location(path, false) {
                        location.push(run)
                    }
                }
            } else if name.contains("cb") {
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

        print_results(
            &mut crowbar,
            &mut nullable_first_20,
            &mut nullable,
            &mut location,
        );

        if crowbar.size() > 0 && nullable.size() > 0 {
            plot::plot(&mut nullable_first_20, &mut crowbar).unwrap();
            plot::plot_nullable(&mut nullable).unwrap();
        }

        if loc {
            let mut f = fs::File::create("loc-runs.txt").unwrap();
            f.write_all(&location.format().into_bytes()).unwrap();
            f.flush().unwrap();
        }
    }

    Ok(())
}
