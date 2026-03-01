fn main() {
    println!("Hello, world!");

    otra_funcion(12);
    funcion_varios_argumentos(6, 'a');
}

fn otra_funcion(x: i32) {
    println!("El valor de 'x' es {x}");
}

fn funcion_varios_argumentos(x: u32, c: char) {
    println!("El valor de los argumentos es {x}{c}");
}