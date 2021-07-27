// Part 1: V2

use std::{io,env};
use std::cmp::Ordering;

fn main() -> io::Result<()> {
    // Get target from command line
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 { panic!("You are missing a param"); }
    let target = args[1].parse::<i32>().unwrap();

    // Read all input
    let mut input = String::new();
    while io::stdin().read_line(&mut input)? != 0 {}

    // Split into lines
    let split = input.split("\n");

    // Convert to a list of i32
    let mut v: Vec<i32> = Vec::new();
    for s in split {
        if !s.is_empty() {
            v.push(s.parse::<i32>().unwrap());
        }
    }

    // Find 3 numbers that sum to target
    let mut i: usize = 0;
    let mut j: usize = 1;
    let mut k: usize = 2;
    let last_element = v.len() - 1;
    loop  {
        match (v[i] + v[j] + v[k]).cmp(&target) {
            Ordering::Equal => {
                println!("Result: {}", v[i] * v[j] * v[k]);
                return Ok(());
            },
            _ => {
                k += 1;
                // k wraps
                if k > last_element {
                    j += 1;
                    // j wraps
                    if j > last_element - 1 {
                        i += 1;
                        // i wraps
                        if i == last_element - 2 {
                            break;
                        }
                        j = i + 1; // j wraps to next after i
                    }
                    k = j + 1; // k wraps to next after j
                }
            }
        }
    }
    println!("Fail");
    Ok(())
}