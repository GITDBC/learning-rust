use std::println;

const UMBRAL: u8 = 5;

fn main() {
    let mut contador: u8 = 0;

    let resultado: u8 = loop {
        contador += 1;

        if contador == UMBRAL {
            // Podemos devolver valor utilizando el break
            // También podemos utilizar return, pero nos saldríamos
            // de la función main, y lo que queremos es sacar el valor
            // del loop
            break contador.pow(2);
        }
    };

    println!("El resultado es: {}", resultado);
}