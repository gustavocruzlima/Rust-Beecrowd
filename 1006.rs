use std::io;

fn main(){
    let a: f64;
    let b: f64;
    let c: f64;

    let mut inputa = String::new();
    let mut inputb = String::new();
    let mut inputc = String::new();

    io::stdin().read_line(&mut inputa).expect("error A");
    io::stdin().read_line(&mut inputb).expect("error B");
    io::stdin().read_line(&mut inputc).expect("error C");

    a = inputa.trim().parse().expect("error on changing A");
    b = inputb.trim().parse().expect("error on changing B");
    c = inputc.trim().parse().expect("error on changing C");

    let media = ((a*2.0) + (b*3.0) + (c*5.0))/10.0;

    println!("MEDIA = {:.1}",media);    
}