use std::io;

fn main(){
    let a: i32;
    let b: i32;
    let c: i32;
    let d: i32;

    let mut inputa = String::new();
    let mut inputb = String::new();
    let mut inputc = String::new();
    let mut inputd = String::new();

    io::stdin().read_line(&mut inputa).expect("error A");
    io::stdin().read_line(&mut inputb).expect("error B");
    io::stdin().read_line(&mut inputc).expect("error C");
    io::stdin().read_line(&mut inputd).expect("error D");

    a = inputa.trim().parse().expect("error on changing A");
    b = inputb.trim().parse().expect("error on changing B");
    c = inputc.trim().parse().expect("error on changing C");
    d = inputd.trim().parse().expect("error on changing D");

    let diferenca = a*b-c*d;

    println!("DIFERENCA = {}",diferenca);    
}