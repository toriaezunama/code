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

    // Sort ascending
    v.sort();

    // Find 2 numbers that sum to target
    let mut i: usize = 0;
    let mut j: usize = v.len() - 1;
    while i < j  {
        match (v[i] + v[j]).cmp(&target) {
            Ordering::Equal => break,
            Ordering::Greater => j -= 1,
            Ordering::Less => i += 1,
        }        
    }
    // Results
    // println!("{} {} {:#?}", v[i], v[j], v[i] + v[j]);
    println!("Result: {}", v[i] * v[j]);
    Ok(())
}
