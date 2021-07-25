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

    let mut i: i32 = 0;
    let mut j: usize = v.len();

    println!("{} {} {:#?}", i, j, v);
    Ok(())
}
