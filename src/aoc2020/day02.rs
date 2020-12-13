use std::{error::Error, fs};

struct Rule<'a> {
    pub left_num: usize,
    pub right_num: usize,
    pub rule_char: &'a str,
    pub password: &'a str,
}

impl Rule<'_> {
    pub fn new(rule_entry: &str) -> Result<Rule, Box<dyn Error>> {
        let rule: Vec<&str> = rule_entry.split(' ').collect();
        let num_range: Vec<&str> = rule[0].split('-').collect();
        let left_num: usize = num_range[0].parse()?;
        let right_num: usize = num_range[1].parse()?;
        let rule_char = &rule[1][..1];
        let password = rule[2];

        Ok(Rule {
            left_num: left_num,
            right_num: right_num,
            rule_char: rule_char,
            password: password,
        })
    }
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string("files/day02_input.txt")?;

    let lines: Vec<&str> = contents.lines().collect();

    part1(&lines)?;
    part2(&lines)?;

    Ok(())
}

fn part1(lines: &Vec<&str>) -> Result<(), Box<dyn Error>> {
    let mut num_valid = 0;
    for line in lines {
        let rule = Rule::new(line)?;
        let char_occurances = rule.password.matches(rule.rule_char).count();
        if char_occurances >= rule.left_num && char_occurances <= rule.right_num {
            num_valid += 1;
        }
    }

    println!("day02-1: {}", num_valid);

    Ok(())
}

fn part2(lines: &Vec<&str>) -> Result<(), Box<dyn Error>> {
    let mut num_valid = 0;
    for line in lines {
        let rule = Rule::new(line)?;
        let first = rule.password.as_bytes()[rule.left_num - 1] == rule.rule_char.as_bytes()[0];
        let second = rule.password.as_bytes()[rule.right_num - 1] == rule.rule_char.as_bytes()[0];
        if first ^ second {
            num_valid += 1;
        }
    }

    println!("day02-2: {}", num_valid);

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
