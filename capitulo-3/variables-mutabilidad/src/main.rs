// Esto no compila porque la variable no es mutable
// fn main() {
//     let variable = 5;
//     println!("El valor de la variable es {}", variable);

//     variable = 6;
//     println!("El valor de la variable es {}", variable);
// }

const NUM_PI: f32 = 3.14159;

fn main() {
    let mut variable = 5;
    println!("El valor de la variable es {}", variable); // 5

    variable = 6;
    println!("El valor de la variable es {}", variable); // 6

    println!("El valor de la constante es {}", NUM_PI); // 3.14159

    // Uso de shadowing
    let variable = 7;
    let variable = variable + 10;
    println!("El valor de la variable es {}", variable); // 17

    {
        let variable = variable * 2;
        println!("El valor de la variable es {}", variable); // 17 * 2 = 34
    }

    println!("El valor de la variable es {}", variable); // 17
}