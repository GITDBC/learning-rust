use std::println;

fn multiplicar_valores(input_a: i32, input_b: i32) -> i32 {
    // En Rust no es necesario utilizar return al final, ni poner el ;
    input_a * input_b
}

fn multiplicar_valores_v2(input_a: i32, input_b: i32) -> i32 {
    // Esto es otra forma
    return input_a * input_b;
}

fn main() {
    let input_a: i32 = 2;
    let input_b: i32 = 4;

    println!("Resultado: {}", multiplicar_valores(input_a, input_b));
    println!("Resultado: {}", multiplicar_valores_v2(input_a, input_b));

    // Because Rust is an expression-based language, this is an important
    // distinction to understand. Statements are instructions that perform
    // some action and do not return a value. Expressions evaluate to a
    // resultant value.
    // Ejemplo: let y = 6 statement does not return a value
    // let x = (let y = 6); -> Esto no funciona

    // Esto es una expresión
    let y = {
        let x: i32 = 6; // statement
        x.pow(2) // expresion
    };

    println!("Valor de y: {}", y);
}