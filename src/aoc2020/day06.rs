use std::{collections::HashSet, error::Error, fs};

pub fn run() -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string("files/day06_input.txt")?;

    let lines: Vec<&str> = contents.lines().collect();
    part1(&lines)?;
    part2(&lines)?;

    Ok(())
}

fn part1(lines: &Vec<&str>) -> Result<(), Box<dyn Error>> {
    let mut current_group_answers: Vec<&str> = Vec::new();
    let mut total_count = 0;
    for line in lines {
        if line.is_empty() {
            let mut answers = HashSet::new();
            for answer in &current_group_answers {
                for c in answer.chars() {
                    answers.insert(c);
                }
            }
            total_count += answers.len();
            current_group_answers.clear();
        } else {
            current_group_answers.push(line);
        }
    }

    println!("day06-1: {}", total_count);

    Ok(())
}

fn part2(lines: &Vec<&str>) -> Result<(), Box<dyn Error>> {
    let mut current_group_answers: Vec<&str> = Vec::new();
    let mut total_count = 0;
    for line in lines {
        if line.is_empty() {
            let mut answers = HashSet::new();
            let first_answer = current_group_answers[0];
            for c in first_answer.chars() {
                let mut all_yes = true;
                if current_group_answers.len() > 1 {
                    for i in 1..current_group_answers.len() {
                        if !current_group_answers[i].contains(c) {
                            all_yes = false;
                        }
                    }
                }

                if all_yes {
                    answers.insert(c);
                }
            }
            
            total_count += answers.len();
            current_group_answers.clear();
        } else {
            current_group_answers.push(line);
        }
    }

    println!("day06-2: {}", total_count);

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
