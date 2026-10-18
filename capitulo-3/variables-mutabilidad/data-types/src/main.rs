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
    
    println!("Arquitectura de {} bits", usize::MAX.to_be_bytes().len() * 8);
    println!("Valor máximo de usize: {}", usize::MAX);
    println!("Valor mínimo de usize: {}", usize::MIN);

    println!("Valor máximo de isize: {}", isize::MAX);
    println!("Valor mínimo de isize: {}", isize::MIN);
}