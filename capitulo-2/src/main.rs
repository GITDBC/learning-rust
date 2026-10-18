use rand::random_range;
use std::cmp::Ordering;
use std::io;

const NUM_INTENTOS: u8 = 3;
const MAX_VALUE: u8 = 10;
const MIN_VALUE: u8 = 0;

fn solicitar_entrada_usuario() -> u8 {
    println!("Introduce un valor entre {} y {}: ", MIN_VALUE, MAX_VALUE);

    // Bucle infinito
    loop {
        // El String es un tipo de datos complejo, que no conocemos la cantidad que va a
        // ocupar en compilación, y que está en el heap.
        // Creamos un string vacío:
        let mut entrada_usuario: String = String::new();

        // Las referencias son inmutables por defecto, necesitamos pasar una referencia
        // mutable para el buffer de lectura del input del usuario.
        io::stdin()
            .read_line(&mut entrada_usuario)
            .expect("Error al leer el valor de entrada.");

        // Utilizamos trim para eliminar los espacio en blanco, y parse para parsear
        // realizando un casting a tipo de u8
        match entrada_usuario.trim().parse::<u8>() {
            // El valor se ha parseado correctamente y está dentro del rango
            Ok(valor_entrada) if (valor_entrada >= MIN_VALUE) && (valor_entrada <= MAX_VALUE) => {
                return valor_entrada;
            }
            // El valor se ha parseado correctamente pero no está dentro del rango
            Ok(_) => {
                println!("Introduce un valor entre {} y {}: ", MIN_VALUE, MAX_VALUE);
                continue; // No sería necesario poner
            }
            // El valor no se ha parseado correctamente
            Err(_) => {
                println!("Entrada no válida. Introduce un número entero:");
                continue; // No sería necesario poner
            }
        }
    }
}

fn juego() {
    println!("Adivina el número :)");
    println!("Tienes {} intentos.", NUM_INTENTOS);

    let mut entrada_usuario = solicitar_entrada_usuario();

    let valor_aleatorio: u8 = random_range(MIN_VALUE..=MAX_VALUE);

    let mut num_intentos_usuario: u8 = 0;

    loop {
        if num_intentos_usuario < NUM_INTENTOS - 1 {
            match entrada_usuario.cmp(&valor_aleatorio) {
                Ordering::Less => {
                    println!("Has fallado :(");
                    println!("Introduce un valor más grande.")
                }
                Ordering::Greater => {
                    println!("Has fallado :(");
                    println!("Introduce un valor más pequeño");
                }
                Ordering::Equal => {
                    println!("Has ganado :)");
                    break;
                }
            }
        } else {
            if entrada_usuario == valor_aleatorio {
                println!("Has ganado :)");
            } else {
                println!("Has perdido el juego :(.");
                println!("El valor a predecir era {}", valor_aleatorio);
            }

            break;
        }

        num_intentos_usuario += 1;

        println!(
            "Intentos restantes {}.",
            NUM_INTENTOS - num_intentos_usuario
        );

        entrada_usuario = solicitar_entrada_usuario();
    }
}

fn main() {
    juego();
}
