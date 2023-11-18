use std::io;

fn main(){
    // variáveis de input
    let mut input1 = String::new();

    // vetores para gravar valores
    let mut v1: Vec<f64> = Vec::new();

    // leitura de valores
    io::stdin().read_line(&mut input1).expect("error A");

    // quebra nos espaços e insert nos vetores
    for i in input1.split_whitespace(){
        v1.push(i.trim().parse().expect("error"));
    }

    // atribuição de valores em variáveis
    let a = &v1[0];
    let b = &v1[1];
    let c = &v1[2];
    let pi = 3.14159;

    // cálculos
    let triangulo = (a*c)/2.0;
    let circulo = pi * c.powf(2.0);
    let trapezio = (a+b)*c/2.0;
    let quadrado = b*b;
    let retangulo = a*b;

    println!("TRIANGULO: {:.3}",triangulo);
    println!("CIRCULO: {:.3}",circulo);
    println!("TRAPEZIO: {:.3}",trapezio); 
    println!("QUADRADO: {:.3}",quadrado);
    println!("RETANGULO: {:.3}",retangulo);
}
