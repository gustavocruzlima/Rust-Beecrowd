use std::io;

fn main(){
    let a: f64;
    let b: f64;

    let mut inputa = String::new();
    let mut inputb = String::new();

    io::stdin().read_line(&mut inputa).expect("error A");
    io::stdin().read_line(&mut inputb).expect("error B");

    a = inputa.trim().parse().expect("error on changing A");
    b = inputb.trim().parse().expect("error on changing B");

    let media = ((a*3.5) + (b*7.5))/11.0;

    println!("MEDIA = {:.5}",media);    
}