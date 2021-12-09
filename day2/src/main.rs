use std::{
    env,
    fs::File,
    io::{BufRead, BufReader, Seek, SeekFrom},
};

use anyhow::Context;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
enum Action {
    Forward(u32),
    Up(u32),
    Down(u32),
}

#[derive(Debug)]
struct Coordinates {
    position: u32,
    depth: u32,
}

fn main() -> anyhow::Result<()> {
    let path = env::args()
        .nth(1)
        .context("Expected path to input as first arg")?;

    let mut file = File::open(&path).with_context(|| format!("Could not open {}", path))?;

    let without_aim_res = without_aim(&file)?;
    println!("Without aim: {:?}", without_aim_res);

    file.seek(SeekFrom::Start(0))?;

    let with_aim_res = with_aim(&file)?;
    println!("With aim: {:?}", with_aim_res);

    Ok(())
}

fn without_aim(file: &File) -> anyhow::Result<Coordinates> {
    let reader = BufReader::new(file);

    let mut coords = Coordinates {
        position: 0,
        depth: 0,
    };
    for line in reader.lines() {
        let line = line?;
        let action = parse_line(&line)?;

        match action {
            Action::Forward(n) => {
                coords.position += n;
            }
            Action::Up(n) => {
                coords.depth -= n;
            }
            Action::Down(n) => {
                coords.depth += n;
            }
        }
    }

    Ok(coords)
}

fn with_aim(file: &File) -> anyhow::Result<Coordinates> {
    let reader = BufReader::new(file);

    let mut aim = 0;
    let mut coords = Coordinates {
        position: 0,
        depth: 0,
    };
    for line in reader.lines() {
        let line = line?;
        let action = parse_line(&line)?;

        match action {
            Action::Forward(n) => {
                coords.position += n;
                coords.depth += n * aim;
            }
            Action::Up(n) => {
                aim -= n;
            }
            Action::Down(n) => {
                aim += n;
            }
        }
    }

    Ok(coords)
}

fn parse_line(line: &str) -> anyhow::Result<Action> {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"^(?P<action>forward|up|down) (?P<number>\d+)$").unwrap();
    }

    let captures = RE
        .captures(line)
        .with_context(|| format!("Line '{}' does not follow correct format", line))?;

    let number: u32 = captures
        .name("number")
        .unwrap()
        .as_str()
        .parse()
        .with_context(|| format!("Could not parse number from line '{}'", line))?;

    let action = match captures.name("action").unwrap().as_str() {
        "forward" => Action::Forward(number),
        "up" => Action::Up(number),
        "down" => Action::Down(number),
        _ => unreachable!("Invalid action"),
    };

    Ok(action)
}
