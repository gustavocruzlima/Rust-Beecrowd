use std::io;

fn main(){
    // variáveis de input
    let mut input1 = String::new();

    // vetores para gravar valores
    let mut v1: Vec<i32> = Vec::new();

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

    // cálculo
    let maior1 = (a + b + (a-b).abs())/2;
    let maior = (c + maior1 + (c-maior1).abs())/2;

    println!("{} eh o maior",maior);

}
