use std::cmp;
use std::fs::File;
use std::io::prelude::*;

pub struct Claim {
    id: String,
    x: i64,
    y: i64,
    w: i64,
    h: i64,
}

// Hardcoded length of input file in lines
static INITIAL_CAPACITY: usize = 1367;

// the max width and height are determined by the get_info function and hardcoded here
const GRID_WIDTH: i64 = 1000;
const GRID_HEIGHT: i64 = 1000;
const GRID_CAPACITY: usize = GRID_WIDTH as usize * GRID_HEIGHT as usize;

pub fn parse_claims(v: &Vec<&str>) -> Vec<Claim> {
    let mut claims: Vec<Claim> = Vec::with_capacity(INITIAL_CAPACITY);

    for val in v {
        if val.len() == 0 {
            continue;
        }
        let val_split: Vec<&str> = val.split(" ").collect();
        let pos_split: Vec<i64> = val_split[2].split(",").map(|x| x.trim_right_matches(":").parse::<i64>().unwrap()).collect();
        let dim_split: Vec<i64> = val_split[3].split("x").map(|x| x.trim().parse::<i64>().unwrap()).collect();

        claims.push(Claim {
            id: val_split[0].to_string(),
            x: pos_split[0],
            y: pos_split[1],
            w: dim_split[0],
            h: dim_split[1],
        });
    }

    claims
}

pub fn get_info() -> std::io::Result<()> {
    let mut file = File::open("input")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let v: Vec<&str> = contents.split("\n").collect();

    let claims = parse_claims(&v);

    let mut max_x: i64 = 0;
    let mut max_y: i64 = 0;

    for claim in &claims {
        max_x = cmp::max(max_x, claim.x + claim.w);
        max_y = cmp::max(max_y, claim.y + claim.h);
    }

    println!("Max x = {}, Max y = {}\n", max_x, max_y);
    Ok(())
}

pub fn part1() -> std::io::Result<()> {
    let mut file = File::open("input")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let v: Vec<&str> = contents.split("\n").collect();

    let claims = parse_claims(&v);

    let mut grid: [i64; GRID_CAPACITY] = [0; GRID_CAPACITY];

    for claim in &claims {
        for y in claim.y..claim.y+claim.h {
            for x in claim.x..claim.x+claim.w {
                grid[(y * GRID_WIDTH + x) as usize] += 1;
            }
        }
    }

    let count = grid.iter().map(|x| (*x > 1i64) as i64 ).fold(0i64, |sum, val| sum + val);

    println!("Answer is {}", count);
    Ok(())
}

pub fn part2() -> std::io::Result<()> {
    let mut file = File::open("input")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let v: Vec<&str> = contents.split("\n").collect();

    let claims = parse_claims(&v);

    let mut grid: [i64; GRID_CAPACITY] = [0; GRID_CAPACITY];

    for claim in &claims {
        for y in claim.y..claim.y+claim.h {
            for x in claim.x..claim.x+claim.w {
                grid[(y * GRID_WIDTH + x) as usize] += 1;
            }
        }
    }

    for claim in &claims {
        let mut bad = false;

        'check: for y in claim.y..claim.y+claim.h {
            for x in claim.x..claim.x+claim.w {
                if grid[(y * GRID_WIDTH + x) as usize] > 1 {
                    bad = true;
                    break 'check;
                }
            }
        }

        if !bad {
            println!("Answer is {}", claim.id);
            break;
        }
    }

    Ok(())
}

fn main() {
    // get_info tells us how big to make our grid
    println!(">>> Info");
    match get_info() {
        Ok(_) => (),
        Err(..) => ()
    }

    println!(">>> Part 1");
    match part1() {
        Ok(_) => println!(">>> Part 1 Succeeded\n"),
        Err(..) => println!(">>> Part 1 Failed\n")
    }

    println!(">>> Part 2");
    match part2() {
        Ok(_) => println!(">>> Part 2 Succeeded"),
        Err(..) => println!(">>> Part 2 Failed")
    }
}
