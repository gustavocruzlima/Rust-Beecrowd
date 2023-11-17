use std::io;

fn main(){
    let mut nome = String::new();
    let salario: f64;
    let vendas: f64;

    let mut inputa = String::new();
    let mut inputb = String::new();

    io::stdin().read_line(&mut nome).expect("error nome");
    io::stdin().read_line(&mut inputa).expect("error A");
    io::stdin().read_line(&mut inputb).expect("error B");

    nome = nome.trim().parse().expect("error!");
    salario = inputa.trim().parse().expect("error!");
    vendas = inputb.trim().parse().expect("error!");

    // trasnformando horas_trabalhadas em double para fazer calculo
    let total:f64 = salario + (vendas* 0.15);

    println!("TOTAL = R$ {:.2}",total); 
}
