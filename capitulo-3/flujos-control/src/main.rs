use std::{io, println};

const LIMITE_SUPERIOR: u8 = 9;
const LIMITE_INFERIOR: u8 = 0;
const CORTE: u8 = 5;

fn main() {
    println!(
        "Introduce un valor en el rango [{}, {}]:",
        LIMITE_INFERIOR, LIMITE_SUPERIOR
    );

    let mut buffer: String = String::new();

    io::stdin()
        .read_line(&mut buffer)
        .expect("Error al leer el input.");

    let valor: u8 = match buffer.trim().parse::<u8>() {
        Ok(valor) if (LIMITE_INFERIOR..=LIMITE_SUPERIOR).contains(&valor) => {
            println!("El valor se encuentra dentro del rango, es válido.");
            valor
        }
        Ok(_) => {
            println!("El valor no se encuentra dentro del rango.");
            println!("El rango es [{}, {}].", LIMITE_INFERIOR, LIMITE_SUPERIOR);
            return;
        }
        Err(_) => {
            println!("Error, el input no es un número.");
            return;
        }
    };

    if valor >= CORTE {
        println!(
            "El valor {} es igual o superior al valor de corte {}.",
            valor, CORTE
        );
    } else {
        println!("El valor {} es menor al valor de corte {}.", valor, CORTE);
    }

    // Podemos utilizar if dentro de lets
    // Tener en cuenta que en este caso los tipos, del if como del else
    // tienen que ser del mismo data type
    let condicion: bool = true;
    let variable: u8 = if condicion { 5 } else { 9 };
    println!("La variable es: {}", variable);
}
