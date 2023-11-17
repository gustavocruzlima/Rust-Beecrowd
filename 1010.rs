use std::io;

fn main(){
    // variáveis de input
    let mut input1 = String::new();
    let mut input2 = String::new();

    // vetores para gravar valores
    let mut v1: Vec<f64> = Vec::new();
    let mut v2: Vec<f64> = Vec::new();

    // leitura de valores
    io::stdin().read_line(&mut input1).expect("error A");
    io::stdin().read_line(&mut input2).expect("error B");

    // quebra nos espaços e insert nos vetores
    for i in input1.split_whitespace(){
        v1.push(i.trim().parse().expect("error"));
    }

    for j in input2.split_whitespace(){
        v2.push(j.trim().parse().expect("error"));
    }

    // atribuição de valores em variáveis
    let num1 = &v1[1];
    let valor1 = &v1[2];
    let num2 = &v2[1];
    let valor2 = &v2[2];

    // cálculo do valor total
    let total = (num1 * valor1) + (num2 * valor2);

    println!("VALOR A PAGAR: R$ {:.2}",total); 
}
