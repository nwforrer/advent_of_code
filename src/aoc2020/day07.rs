use std::error::Error;
use std::fs;
use std::collections::{HashMap, HashSet};
use regex::Regex;

#[derive(Debug)]
struct Bag<'a> {
    color: &'a str,
    child_bag_counts: HashMap<&'a str, u32>,
}

impl Bag<'_> {
    fn new(line: &str) -> Bag {
        let color_re = Regex::new(r"(?P<color>.*?) bag[s]?? contain (?P<contains>.*[.])").unwrap();


        let mut color = "";
        let mut child_bag_counts = HashMap::new();
        if let Some(color_cap) = color_re.captures(line) {
            if let Some(c) = color_cap.name("color") {
                color = c.as_str();
                if let Some(contains) = color_cap.name("contains") {
                    parse_child_bags(contains.as_str(), &mut child_bag_counts);
                }
            }
        }

        Bag {
            color,
            child_bag_counts,
        }
    }
}

fn parse_child_bags<'a>(child_line: &'a str, child_bag_counts: &mut HashMap<&'a str, u32>) {
    if let Ok(child_re) = Regex::new(r"(?P<count>\d) (?P<color>.*?) bag[s]??[,]?\s?(?P<rest>.*[.]?)") {
        if let Some(child_cap) = child_re.captures(child_line) {
            let count = child_cap.name("count").unwrap();
            let color = child_cap.name("color").unwrap();
            child_bag_counts.insert(color.as_str(), count.as_str().parse::<u32>().unwrap());
            if let Some(rest) = child_cap.name("rest") {
                parse_child_bags(rest.as_str(), child_bag_counts);
            }
        }
    }
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string("files/day07_input.txt")?;

    let lines: Vec<&str> = contents.lines().collect();
    part1(&lines)?;
    part2(&lines)?;

    Ok(())
}

fn part1(lines: &Vec<&str>) -> Result<(), Box<dyn Error>> {
    let mut bags = HashMap::new();
    for line in lines {
        let bag = Bag::new(line);
        bags.insert(bag.color, bag);
    }

    let mut target_bag = "shiny gold";
    let mut bags_to_check = Vec::new();
    let mut result = HashSet::new();
    loop {
        for (_color, bag) in bags.iter() {
            if bag.child_bag_counts.contains_key(target_bag) {
                bags_to_check.push(bag.color);
                result.insert(bag.color);
            }
        }
        if bags_to_check.is_empty() {
            break;
        }
        target_bag = bags_to_check.remove(0);
    }

    println!("day07-01: {}", result.len());

    Ok(())
}

fn part2(lines: &Vec<&str>) -> Result<(), Box<dyn Error>> {
    let mut bags = HashMap::new();
    for line in lines {
        let bag = Bag::new(line);
        bags.insert(bag.color, bag);
    }

    let mut result = 0;
    let mut bags_to_check = HashSet::new();
    let mut target_bag = "shiny gold";
    loop {
        match bags.get(target_bag) {
            Some(bag) => {
                bags_to_check.insert(bag.color);

            },
            None => break
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        run().unwrap();
    }

    #[test]
    fn test_part1() -> Result<(), Box<dyn Error>> {
        let contents = fs::read_to_string("files/day07_sample.txt")?;
        let lines: Vec<&str> = contents.lines().collect();

        part1(&lines)
    }

    #[test]
    fn test_part2() -> Result<(), Box<dyn Error>> {
        let contents = fs::read_to_string("files/day07_sample.txt")?;
        let lines: Vec<&str> = contents.lines().collect();

        part2(&lines)
    }

    #[test]
    fn test_regex_one_bag() {
        let re = Regex::new(r"(?P<color>.*?) bag[s]?? contain (?P<count>\d) (?P<child_color>.*?) bag[s]??.").unwrap();
        let input = "bright white bags contain 1 shiny gold bag.";
        let cap = re.captures(input).unwrap();
        assert_eq!(cap.name("color").unwrap().as_str(), "bright white");
        assert_eq!(cap.name("count").unwrap().as_str().parse::<u32>().unwrap(), 1);
        assert_eq!(cap.name("child_color").unwrap().as_str(), "shiny gold")
    }

    #[test]
    fn test_regex_two_bags() {
        let color_re = Regex::new(r"(?P<color>.*?) bag[s]?? contain (?P<contains>.*[.])").unwrap();
        let child_re = Regex::new(r"(?P<count>\d) (?P<color>.*?) bag[s]??[,]?? (?P<rest>.*?)[.]").unwrap();
        let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.";
        let color_cap = color_re.captures(input).unwrap();
        if let Some(contains) = color_cap.name("contains") {
            let child_cap = child_re.captures(contains.as_str()).unwrap();
            assert_eq!(child_cap.name("count").unwrap().as_str().parse::<u32>().unwrap(), 1);
            assert_eq!(child_cap.name("color").unwrap().as_str(), "bright white");
            assert_eq!(child_cap.name("rest").unwrap().as_str(), "2 muted yellow bags");
        }
    }

    #[test]
    fn test_bag_creation() {
        let bag = Bag::new("light red bags contain 1 bright white bag, 2 muted yellow bags.");
        assert_eq!(bag.color, "light red");
        assert_eq!(bag.child_bag_counts.len(), 2);
        assert_eq!(bag.child_bag_counts.get("bright white"), Some(&1));
        assert_eq!(bag.child_bag_counts.get("muted yellow"), Some(&2));
    }
}
