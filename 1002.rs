use std::io;

fn main(){
    let pi = 3.14159;

    let num:f64;
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("not a valid string"); 
    num = input.trim().parse().expect("Not a valid number");

    let a = pi * (num * num);

    println!("A={:.4}",a);
    
}