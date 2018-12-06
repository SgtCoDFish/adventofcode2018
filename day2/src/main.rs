use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;


pub fn part1() -> std::io::Result<()> {
    let mut file = File::open("input")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let v: Vec<&str> = contents.split("\n").collect();

    let mut twos: i32 = 0;
    let mut threes: i32 = 0;

    for val in &v {
        if val.len() == 0 {
            continue;
        }

        let mut incr_two: bool = false;
        let mut incr_three: bool = false;

        let mut map = HashMap::new();

        for c in val.chars() {
            let count = map.entry(c).or_insert(0);
            *count += 1;
        }

        for value in map.values() {
            if *value == 2 {
                incr_two = true;
            } else if *value == 3 {
                incr_three = true;
            }
        }

        if incr_two {
            twos += 1;
        }

        if incr_three {
            threes += 1;
        }
    }

    println!("Answer is {}", twos * threes);
    Ok(())
}

pub fn part2() -> std::io::Result<()> {
    let mut file = File::open("input")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let v: Vec<&str> = contents.split("\n").collect();

    'outer: for (i, val) in v.iter().enumerate() {
        for inner_val in &v[i..] {
            let distance = hamming_distance(val, inner_val);
            if distance == 1 {
                println!("Answer is {} {}", val, inner_val);
                break 'outer;
            }
        }
    }

    Ok(())
}

pub fn hamming_distance(s1: &str, s2: &str) -> i32 {
    if s1.len() != s2.len() {
        // technically undefined but we don't care here
        return 1000000
    }

    s1.chars().zip(s2.chars()).map(|(c1, c2)| if c1 == c2 {0} else {1}).fold(0i32, |sum, val| sum + val)
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

