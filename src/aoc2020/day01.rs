use std::error::Error;
use std::fs;

pub fn run() -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string("files/day01_input.txt")?;

    let lines: Vec<&str> = contents.lines().collect();
    part1(&lines)?;
    part2(&lines)?;
    Ok(())
}

fn part1(lines: &Vec<&str>) -> Result<(), Box<dyn Error>> {
    for i in 0..lines.len() {
        for j in 1..lines.len() - 1 {
            let one: u32 = lines[i].parse()?;
            let two: u32 = lines[j].parse()?;
            if one + two == 2020 {
                println!("day01-1: {}", one * two);
                return Ok(());
            }
        }
    }

    println!("day01-1: No result");
    Ok(())
}

fn part2(lines: &Vec<&str>) -> Result<(), Box<dyn Error>> {
    for i in 0..lines.len() {
        for j in 1..lines.len() - 1 {
            for k in 2..lines.len() - 2 {
                let one: u32 = lines[i].parse()?;
                let two: u32 = lines[j].parse()?;
                let three: u32 = lines[k].parse()?;
                if one + two + three == 2020 {
                    println!("day01-2: {}", one * two * three);
                    return Ok(());
                }
            }
        }
    }

    println!("day01-2: No result");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn part1() {
        run().unwrap();
    }
}