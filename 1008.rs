use std::io;

fn main(){
    let numero: i32;
    let horas_trabalhadas: i32;
    let salario: f64;

    let mut inputa = String::new();
    let mut inputb = String::new();
    let mut inputc = String::new();

    io::stdin().read_line(&mut inputa).expect("error A");
    io::stdin().read_line(&mut inputb).expect("error B");
    io::stdin().read_line(&mut inputc).expect("error C");

    numero = inputa.trim().parse().expect("error on changing numero");
    horas_trabalhadas = inputb.trim().parse().expect("error on changing horas_trabalhadas");
    salario = inputc.trim().parse().expect("error on changing salario");

    // trasnformando horas_trabalhadas em double para fazer calculo
    let valor_salario:f64 = horas_trabalhadas as f64 * salario;

    println!("NUMBER = {}",numero); 
    println!("SALARY = U$ {:.2}",valor_salario); 
}
