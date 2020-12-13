use std::{error::Error, fs};

struct Passport<'a> {
    birth_year: &'a str,
    issue_year: &'a str,
    expire_year: &'a str,
    height: &'a str,
    hair_color: &'a str,
    eye_color: &'a str,
    passport_id: &'a str,
}

impl Passport<'_> {
    pub fn new(lines: Vec<&str>) -> Result<Passport, Box<dyn Error>> {
        let mut birth_year = "";
        let mut issue_year = "";
        let mut expire_year = "";
        let mut height = "";
        let mut hair_color = "";
        let mut eye_color = "";
        let mut passport_id = "";
        for line in lines {
            let line_pairs: Vec<&str> = line.split(' ').collect();
            for pair in line_pairs {
                let keyvalue: Vec<&str> = pair.split(':').collect();
                let key = keyvalue[0];
                let value = keyvalue[1];
                match key {
                    "byr" => {
                        birth_year = value;
                    }
                    "iyr" => {
                        issue_year = value;
                    }
                    "eyr" => {
                        expire_year = value;
                    }
                    "hgt" => {
                        height = value;
                    }
                    "hcl" => {
                        hair_color = value;
                    }
                    "ecl" => {
                        eye_color = value;
                    }
                    "pid" => {
                        passport_id = value;
                    }
                    &_ => {}
                }
            }
        }

        Ok(Passport {
            birth_year,
            issue_year,
            expire_year,
            height,
            hair_color,
            eye_color,
            passport_id,
        })
    }

    pub fn check_required(&self) -> bool {
        self.birth_year != ""
            && self.issue_year != ""
            && self.expire_year != ""
            && self.height != ""
            && self.hair_color != ""
            && self.eye_color != ""
            && self.passport_id != ""
    }

    pub fn validate(&self) -> bool {
        let mut valid = true;
        valid &= validate_year(&self.birth_year, 4, 1920, 2002);
        valid &= validate_year(&self.issue_year, 4, 2010, 2020);
        valid &= validate_year(&self.expire_year, 4, 2020, 2030);
        valid &= validate_height(&self.height);
        valid &= validate_hair_color(&self.hair_color);
        valid &= validate_eye_color(&self.eye_color);
        valid &= validate_passport_id(&self.passport_id);

        return valid;
    }
}

fn validate_height(height: &str) -> bool {
    let mut valid = true;
    if height.ends_with("cm") {
        let height = &height[..height.len() - 2];
        match height.parse::<u32>() {
            Ok(height) => {
                if height < 150 || height > 193 {
                    valid = false;
                }
            }
            Err(_) => {
                valid = false;
            }
        }
    } else if height.ends_with("in") {
        let height = &height[..height.len() - 2];
        match height.parse::<u32>() {
            Ok(height) => {
                if height < 59 || height > 76 {
                    valid = false;
                }
            }
            Err(_) => {
                valid = false;
            }
        }
    } else {
        valid = false;
    }

    valid
}

fn validate_hair_color(color: &str) -> bool {
    let mut valid = true;

    if color.len() != 7 {
        valid = false;
    } else {
        let first = &color[0..1];
        if first != "#" {
            valid = false;
        } else {
            for c in color[1..].chars() {
                if !c.is_alphabetic() && !c.is_numeric() {
                    valid = false;
                    break;
                }
            }
        }
    }

    valid
}

fn validate_eye_color(color: &str) -> bool {
    let mut valid = true;
    match color {
        "amb" => {}
        "blu" => {}
        "brn" => {}
        "gry" => {}
        "grn" => {}
        "hzl" => {}
        "oth" => {}
        &_ => {
            valid = false;
        }
    }
    valid
}

fn validate_passport_id(id: &str) -> bool {
    let mut valid = true;

    if id.len() != 9 {
        valid = false;
    } else {
        if let Err(_) = id.parse::<u32>() {
            valid = false;
        }
    }

    valid
}

fn validate_year(year: &str, length: usize, min: u32, max: u32) -> bool {
    let mut valid = true;
    if year.len() != length {
        valid = false;
    } else {
        match year.parse::<u32>() {
            Ok(year) => {
                if year < min || year > max {
                    valid = false;
                }
            }
            Err(_) => {
                valid = false;
            }
        }
    }

    valid
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string("files/day04_input.txt")?;

    let lines: Vec<&str> = contents.lines().collect();
    part1(&lines)?;
    part2(&lines)?;

    Ok(())
}

fn parse_passports<'a>(lines: &'a Vec<&str>) -> Result<Vec<Passport<'a>>, Box<dyn Error>> {
    let mut passport_lines: Vec<&str> = Vec::new();
    let mut passports: Vec<Passport> = Vec::new();
    for line in lines {
        if line.trim().is_empty() {
            let passport = Passport::new(passport_lines.clone())?;
            passports.push(passport);
            passport_lines.clear();
        } else {
            passport_lines.push(line);
        }
    }

    if passport_lines.len() > 0 {
        let passport = Passport::new(passport_lines)?;
        passports.push(passport);
    }

    Ok(passports)
}

fn part1(lines: &Vec<&str>) -> Result<(), Box<dyn Error>> {
    let passports = parse_passports(lines)?;

    let mut num_valid = 0;
    for passport in passports {
        if passport.check_required() {
            num_valid += 1;
        }
    }

    println!("day04-1: {}", num_valid);

    Ok(())
}

fn part2(lines: &Vec<&str>) -> Result<(), Box<dyn Error>> {
    let passports = parse_passports(lines)?;

    let mut num_valid = 0;
    for passport in passports {
        if passport.validate() {
            num_valid += 1;
        }
    }

    println!("day04-2: {}", num_valid);

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
