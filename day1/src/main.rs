use std::fs::File;
use std::collections::HashSet;
use std::io::prelude::*;

pub fn part1() -> std::io::Result<()> {
    let mut file = File::open("input")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let v = contents.split("\n").map(|x| x.parse::<i64>().unwrap_or(0)).fold(0i64, |sum, val| sum + val);

    println!("Answer is {}", v);
    Ok(())
}

pub fn part2() -> std::io::Result<()> {
    let mut file = File::open("input")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let v: Vec<i64> = contents.split("\n").map(|x| x.parse::<i64>().unwrap_or(0)).collect();

    let mut sum = 0i64;
    let mut seen = HashSet::new();
    seen.insert(0i64);
    loop {
        for val in &v {
            if *val == 0 {
                continue;
            }

            sum += *val;

            if seen.contains(&sum) {
                println!("Answer is {}", sum);
                return Ok(());
            }

            seen.insert(sum);
        }
    }
}

fn main() {
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
