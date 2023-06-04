extern crate rand;
use std::io;

fn main() {
    loop {
        println!("Digite um numero: ");
        let mut value: String = String::new();
        io::stdin().read_line(&mut value).expect("Uma falha no sistema ocorreu!");

        let value = match value.trim().parse::<i32>(){
            Ok(number) => number,
            Err(_) => {
                println!("Error: o Valor digitado não é um numero inteiro válido!");
                continue;
            }
        };

        if verificar(value) {
           println!("O valor {value} é par! Continue o Loop"); 
        } else {
            println!("O valor {value} é impar! Encerre o Loop");
            break;
        }
    } 
   
}
fn verificar(a:i32) -> bool {
    let result = if a % 2 == 0 {true} else {false};
    return result;    
}