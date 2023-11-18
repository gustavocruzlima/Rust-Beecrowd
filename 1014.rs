use std::io;

fn main(){
    let x: i32;
    let y: f64;

    let mut inputa = String::new();
    let mut inputb = String::new();

    io::stdin().read_line(&mut inputa).expect("error A");
    io::stdin().read_line(&mut inputb).expect("error B");

    x = inputa.trim().parse().expect("error on A");
    y = inputb.trim().parse().expect("error on B");

    let consumo:f64 = x as f64 / y;

    println!("{:.3} km/l",consumo);    
}