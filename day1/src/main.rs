use std::{
    env,
    fs::File,
    io::{BufRead, BufReader, Seek, SeekFrom},
};

use anyhow::{bail, Context};
use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    let path = env::args()
        .nth(1)
        .context("Expected path to input as first arg")?;

    let mut file = File::open(&path).with_context(|| format!("Could not open {}", path))?;

    println!("Number of single increases: {}", single_increases(&file)?);

    file.seek(SeekFrom::Start(0))?;

    println!(
        "Number of windowed increases: {}",
        windowed_increases(&file)?
    );

    Ok(())
}

fn single_increases(file: &File) -> anyhow::Result<u32> {
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let mut last: u32 = match lines.next() {
        Some(line) => {
            let line = line.context("Line could not be read")?;

            line.trim()
                .parse()
                .with_context(|| format!("'{}' is not a valid number", line))?
        }
        None => bail!("No lines"),
    };

    let mut counter = 0;
    for line in lines {
        let current = line?.trim().parse()?;
        if current > last {
            counter += 1;
        }

        last = current;
    }

    Ok(counter)
}

fn windowed_increases(file: &File) -> anyhow::Result<u32> {
    let reader = BufReader::new(file);

    let measurements: Vec<u32> = reader
        .lines()
        .filter_map_ok(|f| f.trim().parse().ok())
        .try_collect()?;

    let window_sums: Vec<u32> = measurements
        .into_iter()
        .tuple_windows()
        .map(|(a, b, c)| a + b + c)
        .collect();

    let increases = window_sums
        .iter()
        .zip(window_sums.iter().skip(1))
        .filter(|(a, b)| b > a)
        .count();

    Ok(increases as u32)
}
