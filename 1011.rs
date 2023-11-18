use std::io;

fn main(){
    let mut input1 = String::new();
    let raio: f64;

    io::stdin().read_line(&mut input1).expect("error");

    raio = input1.trim().parse().expect("error!");

    let pi = 3.14159;

    let volume = (4.0/3.0) * pi * (raio.powf(3.0));

    println!("VOLUME = {:.3}",volume); 
}
