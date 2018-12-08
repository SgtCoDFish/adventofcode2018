use std::fs::File;
use std::io::prelude::*;

pub fn part1() -> std::io::Result<()> {
    let mut file = File::open("input")?;
    //let mut file = File::open("input_simple")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut cbytes: Vec<u8>  = contents.into_bytes();

    loop {
        let mut changed = false;

        for i in 0..cbytes.len()-1 {
            let diff = if cbytes[i] > cbytes[i+1] {
                cbytes[i] - cbytes[i+1]
            } else {
                cbytes[i+1] - cbytes[i]
            };

            if diff == 32 {
                //println!("Remove {} and {}", cbytes[i] as char, cbytes[i+1] as char);
                cbytes.remove(i);
                cbytes.remove(i); // we're removing what _was_ i+1 before we deleted i
                changed = true;
                break;
            }
        }

        if !changed {
            break;
        }
    }

    println!("Answer is {}", String::from_utf8(cbytes).unwrap().trim().len());
    Ok(())
}

pub fn part2() -> std::io::Result<()> {
    let mut file = File::open("input")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    for char_idx in 65u8..91u8 { // 'A' to 'Z'
        let mut cbytes: Vec<u8> = contents.bytes().filter(|&x| x != char_idx && x != (char_idx + 32)).collect();

        loop {
            let mut changed = false;

            for i in 0..cbytes.len()-1 {
                let diff = if cbytes[i] > cbytes[i+1] {
                    cbytes[i] - cbytes[i+1]
                } else {
                    cbytes[i+1] - cbytes[i]
                };

                if diff == 32 {
                    //println!("Remove {} and {}", cbytes[i] as char, cbytes[i+1] as char);
                    cbytes.remove(i);
                    cbytes.remove(i); // we're removing what _was_ i+1 before we deleted i
                    changed = true;
                    break;
                }
            }

            if !changed {
                break;
            }
        }

        println!("Removing {} gives {}", char_idx as char, cbytes.len() - 1); // -1 to ignore \n on the end
    }

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
