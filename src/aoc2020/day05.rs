use std::{error::Error, fs};

struct BoardingPass<'a> {
    pub bsp: &'a str,
    pub row: u32,
    pub col: u32,
    pub seat_id: u32,
}

impl BoardingPass<'_> {
    fn new(bsp: &str) -> BoardingPass {
        let row = parse_bsp(&bsp[..7], 0, 127, "F", "B");
        let col = parse_bsp(&bsp[7..], 0, 7, "L", "R");
        let seat_id = row * 8 + col;

        BoardingPass {
            bsp,
            row,
            col,
            seat_id,
        }
    }
}

fn parse_bsp(bsp: &str, min_range: u32, max_range: u32, take_lower: &str, take_upper: &str) -> u32 {
    if bsp.len() == 1 {
        if bsp == take_lower {
            return min_range;
        } else {
            return max_range;
        }
    }

    if &bsp[0..1] == take_lower {
        return parse_bsp(&bsp[1..], min_range, min_range + ((max_range - min_range) / 2), take_lower, take_upper);
    } else {
        return parse_bsp(&bsp[1..], min_range + ((max_range - min_range) / 2) + 1, max_range, take_lower, take_upper);
    }
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string("files/day05_input.txt")?;

    let lines: Vec<&str> = contents.lines().collect();
    part1(&lines)?;
    part2(&lines)?;

    Ok(())
}

fn part1(lines: &Vec<&str>) -> Result<(), Box<dyn Error>> {
    let mut highest_seat_id = 0;
    for line in lines {
        let boarding_pass = BoardingPass::new(line);
        if boarding_pass.seat_id > highest_seat_id {
            highest_seat_id = boarding_pass.seat_id;
        }
    }

    println!("day05-1: {}", highest_seat_id);

    Ok(())
}

fn part2(lines: &Vec<&str>) -> Result<(), Box<dyn Error>> {
    let mut taken_seats: Vec<u32> = Vec::new();
    for line in lines {
        let boarding_pass = BoardingPass::new(line);
        taken_seats.push(boarding_pass.seat_id);
    }

    taken_seats.sort();

    let mut my_seat = 0;
    let mut prev_seat: u32 = taken_seats[0];
    for seat in taken_seats {
        if seat - prev_seat > 1 {
            my_seat = seat - 1;
            break;
        }
        prev_seat = seat;
    }

    println!("day05-2: {}", my_seat);

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
    fn test_parse_bsp_row() {
        let result = parse_bsp(&"FBFBBFFRLR"[..7], 0, 127, "F", "B");
        assert_eq!(result, 44);
    }

    #[test]
    fn test_parse_bsp_col() {
        let result = parse_bsp(&"FBFBBFFRLR"[7..], 0, 7, "L", "R");
        assert_eq!(result, 5);
    }
}
