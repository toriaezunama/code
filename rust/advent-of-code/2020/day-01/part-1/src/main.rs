use std::io;

fn main() -> io::Result<()> {
    let mut input = String::new();
    loop {
        match io::stdin().read_line(&mut input) {
            Ok(0) => break,
            Ok(_) => {}
            Err(error) => println!("error: {}", error),
        }
    }
    let split = input.split("\n");
    let mut v: Vec<i32> = Vec::new();
    for s in split {
        if !s.is_empty() {
            v.push(s.parse::<i32>().unwrap());
        }
    }
    v.sort();

    let mut i: usize = 0;
    let mut j: usize = v.len() - 1;

    let target: i32 = 2020;
    while i < j {
        let sum = v[i] + v[j];
        if sum == target {
            break;
        } else if sum > target {
            j -= 1;
        } else if sum < target {
            i += 1;
        }
    }

    println!("{} {} {:#?}", v[i], v[j], v[i] + v[j]);
    println!("Result: {}", v[i] * v[j]);
    Ok(())
}
