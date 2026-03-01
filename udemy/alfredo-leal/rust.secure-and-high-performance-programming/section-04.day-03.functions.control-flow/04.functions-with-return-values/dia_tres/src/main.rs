fn main() {
    println!("Hello, world!");

    otra_funcion(12);
    funcion_varios_argumentos(6, 'a');

    let y = siete();
    println!("El valor de 'y' es: {y}");

    let z = sumar_dos(3);
    println!("El valor de 'z' es: {z}");
}

fn siete() -> i32 {
    7
}

fn sumar_dos(x: i32) -> i32 {
    x + 2
}

fn otra_funcion(x: i32) {
    println!("El valor de 'x' es {x}");
}

fn funcion_varios_argumentos(x: u32, c: char) {
    println!("El valor de los argumentos es {x}{c}");
}