use std::fs::File;
use std::io::prelude::*;
use std::collections::VecDeque;
use std::collections::HashMap;


pub fn smart_insert(vd: &mut VecDeque<char>, c: char) {
    println!("{:?}", vd);

    if vd.len() == 0 {
        println!("adding {} onto back of empty vd", c);
        vd.push_back(c);
        return;
    }

    let mut pos = -1i32;
    let mut greater_any = false;

    for (i, a) in vd.iter().enumerate() {
        if (c as u8) < (*a as u8) {
            println!("{} < {}", c, *a);
            pos = i as i32;
            break;
        } else {
            greater_any = true;
            println!("{} >= {}", c, *a);
        }
    }

    if pos == -1i32 {
        if greater_any {
            println!("adding {} to back", c);
            vd.push_back(c);
        } else {
            println!("adding {} to front", c);
            vd.push_front(c);
        }
    } else {
        println!("adding {} at {}", c, pos);
        vd.insert(pos as usize, c);
    }
}

pub fn part1() -> std::io::Result<()> {
    let mut file = File::open("input")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut result = String::new();

    let v: Vec<&str> = contents.split("\n").collect();

    let mut map: HashMap<char, Vec<char>> = HashMap::new();

    let mut available: VecDeque<char> = VecDeque::new();

    for i in 65u8..91u8 {
        map.insert(i as char, Vec::new());
    }

    for s in v {
        if s.len() == 0 {
            break;
        }

        let mut c: Vec<char> = s.chars().collect();
        let mut entry = map.get_mut(&c[1]).unwrap();
        (*entry).push(c[0]);
    }

    let mut to_clean: Vec<char> = Vec::new();
    for (k, v) in &map {
        if v.len() == 0 {
            smart_insert(&mut available, *k);
            to_clean.push(*k);
        }
    }

    for v in &to_clean {
        map.remove(v);
    }

    while available.len() > 0 {
        println!("{:?}", available);
        let val = available.pop_front().unwrap();
        result.push(val);

        map.remove(&val);

        for (k, v) in &mut map {
            let mut pos = -1i32;
            for (i, c) in v.iter().enumerate() {
                if *c == val {
                    pos = i as i32;
                    break;
                }
            }

            if pos >= 0 {
                println!("removing {} dependency for {}", val, k);
                v.swap_remove(pos as usize);
            }

            if v.len() == 0 && !available.contains(k) {
                println!("Adding {}", *k);
                smart_insert(&mut available, *k);
            }
        }
    }

    println!("{:?}", result);

    Ok(())
}

pub fn part2() -> std::io::Result<()> {
    let mut file = File::open("input")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(())
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
