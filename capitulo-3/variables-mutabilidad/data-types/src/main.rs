use std::io;

const NUM_VALORES_GENERAR: usize = 10;

fn main() {
    // Esto da un error al compilar por overflow
    // let valor: u8 = 255;
    // println!("Valor + 1 {}", valor + 1);

    // Tenemos valores enteros, con y sin signo
    // de 8, 16, 32, 64, 128 bits
    // podemos usar usize, isize para tener los bits de la arquitectura
    let valor: usize = 5;
    let valor_i: isize = -90;
    println!("Valor: {}, valor_i: {}", valor, valor_i);

    println!(
        "Arquitectura de {} bits",
        usize::MAX.to_be_bytes().len() * 8
    );
    println!("Valor máximo de usize: {}", usize::MAX);
    println!("Valor mínimo de usize: {}", usize::MIN);

    println!("Valor máximo de isize: {}", isize::MAX);
    println!("Valor mínimo de isize: {}", isize::MIN);

    let datos: (String, u8) = ("Paquito".to_string(), 65);
    println!("\nNombre: {}, edad: {}", datos.0, datos.1);

    let datos = ("Conchi".to_string(), 60);
    let (nombre, edad) = datos;
    println!("Nombre: {}, edad: {}", nombre, edad);

    let tupla_vacia: () = ();
    println!("Tupla vacia: {:?}", tupla_vacia);

    let lista_valores: [u8; 4] = [1, 2, 3, 4];
    println!("Lista de valores: {:?}", lista_valores);

    let lista_valores: [u8; 5] = [3; 5];
    println!("Lista de valores: {:?}\n", lista_valores);

    let mut vector_valores: Vec<u8> = Vec::new();
    while vector_valores.len() < NUM_VALORES_GENERAR {
        println!("Introduce un valor para rellenar el vector:");

        let mut buffer_usuario: String = String::new();

        io::stdin()
            .read_line(&mut buffer_usuario)
            .expect("Error lectura.");

        let buffer_usuario: u8 = buffer_usuario.trim().parse::<u8>().expect("Error parseo.");

        vector_valores.push(buffer_usuario);
    }

    println!("Lista de valores usuario: {:?}", vector_valores);
}
