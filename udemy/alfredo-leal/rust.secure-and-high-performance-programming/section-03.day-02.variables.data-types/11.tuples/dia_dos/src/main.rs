fn main() {

    /**
     * [ENGLISH]
     * 
     * ==========
     * 
     * [SPANISH]
     * 
     * Shadowing.
     * Tuplas.
     */
    let tupla: (i32, f64, u8) = (300, 3.2, 4);

    let tupla = (300, 3.2, 4);
    let (x, y, z) = tupla;
    println!("El valor de 'y' es: {y}");

    let valor_x = tupla.0;
    let valor_y = tupla.1;
    let valor_z = tupla.2;
    println!("El valor de 'z' es: {valor_z}");
}
