extern crate regex;

use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

use regex::Regex;

pub struct LogEntry<'a> {
    pub year: i32,
    pub month: i32,
    pub day: i32,
    pub hour: i32,
    pub min: i32,
    pub entry: &'a str
}

pub fn part1() -> std::io::Result<()> {
    //let mut file = File::open("simple_input")?;
    let mut file = File::open("input_sorted")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let v: Vec<&str> = contents.trim().split("\n").collect();

    let re = Regex::new(r"\[(\d{4})-(\d{2})-(\d{2}) (\d{2}):(\d{2})\] (.*)").unwrap();
    let guard_re = Regex::new(r"Guard #(\d+) begins shift").unwrap();

    let mut map: HashMap<i32, (i32, [i32; 60])> = HashMap::new();
    let mut guard_id = 0i32;
    let mut start_sleep_time = 0i32;

    let mut sleepiest_guard = 0i32;
    let mut largest_time = 0i32;
    let mut sleepiest_minute = 0i32;

    for val in &v {
        let cap = re.captures(val).unwrap();
        let line = LogEntry {
            year: cap[1].parse::<i32>().unwrap(),
            month: cap[2].parse::<i32>().unwrap(),
            day: cap[3].parse::<i32>().unwrap(),
            hour: cap[4].parse::<i32>().unwrap(),
            min: cap[5].parse::<i32>().unwrap(),
            entry: &cap[6]
        };

        if line.entry.starts_with("Guard #") {
            let parsed_id = guard_re.captures(line.entry).unwrap()[1].parse::<i32>().unwrap();
            if !map.contains_key(&parsed_id) {
                map.insert(parsed_id, (0i32, [0i32; 60]));
            }

            guard_id = parsed_id;
            // println!("New guard #{}", guard_id);
        } else if line.entry.starts_with("falls") {
            start_sleep_time = line.min;
        } else if line.entry.starts_with("wakes") {
            let time = line.min - start_sleep_time;
            let mut mapref = map.get_mut(&guard_id).unwrap();
            mapref.0 += time;
            for i in start_sleep_time..line.min {
                mapref.1[i as usize] += 1;
                // println!("Guard #{} Minute #{} is now {}", guard_id, i, mapref.1[i as usize]);
            }

            if mapref.0 > largest_time {
                largest_time = mapref.0;
                sleepiest_guard = guard_id;
                // println!("{} has new largest {}", guard_id, largest_time);

                let mut pos = 0i32;
                let mut mintmp = 0i32;
                for i in 0..mapref.1.len() {
                    if mapref.1[i] > mintmp {
                        mintmp = mapref.1[i];
                        pos = i as i32;
                    }
                }
                sleepiest_minute = pos;
            }
        }
    }

    println!("Guard {} on minute {} gives answer {}", sleepiest_guard, sleepiest_minute, sleepiest_guard * sleepiest_minute);

    Ok(())
}

pub fn part2() -> std::io::Result<()> {
    //let mut file = File::open("simple_input")?;
    let mut file = File::open("input_sorted")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let v: Vec<&str> = contents.trim().split("\n").collect();

    let re = Regex::new(r"\[(\d{4})-(\d{2})-(\d{2}) (\d{2}):(\d{2})\] (.*)").unwrap();
    let guard_re = Regex::new(r"Guard #(\d+) begins shift").unwrap();

    let mut map: HashMap<i32, [i32; 60]> = HashMap::new();
    let mut guard_id = 0i32;
    let mut start_sleep_time = 0i32;

    let mut sleepiest_guard = 0i32;
    let mut sleepiest_minute_count = 0i32;
    let mut sleepiest_minute_pos = 0i32;

    for val in &v {
        let cap = re.captures(val).unwrap();
        let line = LogEntry {
            year: cap[1].parse::<i32>().unwrap(),
            month: cap[2].parse::<i32>().unwrap(),
            day: cap[3].parse::<i32>().unwrap(),
            hour: cap[4].parse::<i32>().unwrap(),
            min: cap[5].parse::<i32>().unwrap(),
            entry: &cap[6]
        };

        if line.entry.starts_with("Guard #") {
            let parsed_id = guard_re.captures(line.entry).unwrap()[1].parse::<i32>().unwrap();
            if !map.contains_key(&parsed_id) {
                map.insert(parsed_id, [0i32; 60]);
            }

            guard_id = parsed_id;
            // println!("New guard #{}", guard_id);
        } else if line.entry.starts_with("falls") {
            start_sleep_time = line.min;
        } else if line.entry.starts_with("wakes") {
            let mut mapref = map.get_mut(&guard_id).unwrap();
            for i in start_sleep_time..line.min {
                mapref[i as usize] += 1;
                // println!("Guard #{} Minute #{} is now {}", guard_id, i, mapref.1[i as usize]);

                if mapref[i as usize] > sleepiest_minute_count {
                    sleepiest_minute_count = mapref[i as usize];
                    sleepiest_minute_pos = i;
                    sleepiest_guard = guard_id;
                }
            }
        }
    }

    println!("Guard {} had the sleepiest minute {} giving an answer of {}", sleepiest_guard, sleepiest_minute_pos, sleepiest_guard * sleepiest_minute_pos);

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
