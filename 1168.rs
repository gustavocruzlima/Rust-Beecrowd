use std::io;

fn main(){
    let leds: i32;

    let mut input1 = String::new();
   
    io::stdin().read_line(&mut input1).expect("error A");
    
    leds = input1.trim().parse().expect("error on A");
    
    let mut counter = 0;

    loop{
        counter = counter + 1;
        entrada_numero();
        if counter == leds{
            break;
        }
    }
}

fn entrada_numero() -> Vec<u8> {
    let mut numero: i32;
    let mut vec: Vec<u8> = Vec::new();

    let mut input2 = String::new();

    io::stdin().read_line(&mut input2).expect("error B");

    numero = input2.trim().parse().expect("error on B");

    vec = to_digits(numero);

    to_leds(vec);

}

fn to_digits(mut v: i32) -> Vec<u8> {
    let mut buf = [0u8; 20];
    let mut curr = buf.len();

    for digit in buf.iter_mut().rev() {
        let n = v % 10;
        v = v / 10;
        *digit = n as u8;
        curr -= 1;
        if v == 0 {
            
            break;
        };
    }

    return buf[curr..].to_vec()
}

fn to_leds(mut vec: Vec<u8>){
    let mut numero_leds: i32 = 0; 

    for item in vec.iter(){
        match item{
            1 => numero_leds = numero_leds + 2,
            2 => numero_leds = numero_leds + 5, 
        }
    }

    println!("{}", numero_leds);
}
