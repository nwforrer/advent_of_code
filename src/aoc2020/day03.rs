use std::{error::Error, fs};

pub fn run() -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string("files/day03_input.txt")?;

    let lines: Vec<&str> = contents.lines().collect();

    let grid: String = lines.iter().flat_map(|s| s.chars()).collect();
    let grid: Vec<char> = grid.chars().collect();

    part1(&grid, lines[0].len(), contents.lines().count())?;
    part2(&grid, lines[0].len(), contents.lines().count())?;

    Ok(())
}

fn part1(grid: &Vec<char>, width: usize, height: usize) -> Result<(), Box<dyn Error>> {
    let mut num_trees = 0;

    let mut x = 0;
    let mut y = 0;
    while y <= height {
        let cell = grid[(x%width) + (y%height)*width];
        if cell == '#' {
            num_trees += 1;
        }

        x += 3;
        y += 1;
    }

    println!("day03-1: {}", num_trees);

    Ok(())
}

fn part2(grid: &Vec<char>, width: usize, height: usize) -> Result<(), Box<dyn Error>> {
    let slopes: Vec<Vec<u32>> = vec![vec![1, 1], vec![3, 1], vec![5, 1], vec![7, 1], vec![1, 2]];
    let mut answer: u64 = 1;

    for slope in slopes {
        let mut num_trees = 0;

        let mut x: usize = 0;
        let mut y: usize = 0;
        while y <= height {
            let cell = grid[(x%width) + (y%height)*width];
            if cell == '#' {
                num_trees += 1;
            }

            x += slope[0] as usize;
            y += slope[1] as usize;
        }
        answer *= num_trees
    }

    println!("day03-2: {}", answer);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        run().unwrap();
    }
}
