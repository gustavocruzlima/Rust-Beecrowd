use std::io;

fn main(){
    let a: i32;
    let b: i32;

    let mut inputa = String::new();
    let mut inputb = String::new();

    io::stdin().read_line(&mut inputa).expect("error A");
    io::stdin().read_line(&mut inputb).expect("error B");

    a = inputa.trim().parse().expect("error on changing A");
    b = inputb.trim().parse().expect("error on changing B");

    let soma = a + b;

    println!("SOMA = {}", soma);
}