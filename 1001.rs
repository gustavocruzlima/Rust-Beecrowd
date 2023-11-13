use std::io;

fn main() {
     let mut k = String::new();
    io::stdin()
        .read_line(&mut k)
        .expect("Failed to read line");
            let a: i32 = k.trim().parse().expect("Input not an integer");
            
     let mut y = String::new();
    io::stdin()
        .read_line(&mut y)
        .expect("Failed to read line");
            let b: i32 = y.trim().parse().expect("Input not an integer");
            
    let x = a + b;

    println!("X = {}", x);
}