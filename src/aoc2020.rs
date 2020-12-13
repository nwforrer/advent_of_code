use std::error::Error;

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;

pub fn run() -> Result<(), Box<dyn Error>>{
    day01::run()?;
    day02::run()?;
    day03::run()?;
    day04::run()?;
    day05::run()?;
    day06::run()?;
    day07::run()?;

    Ok(())
}