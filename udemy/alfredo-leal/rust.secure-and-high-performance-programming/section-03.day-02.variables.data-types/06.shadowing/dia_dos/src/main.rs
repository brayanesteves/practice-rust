fn main() {

    /**
     * [ENGLISH]
     * 
     * ==========
     * 
     * [SPANISH]
     * 
     * Shadowing.
     * Es declarar en un código, en un método o en un bloque de código variables con el mismo nombre.
     */
    let y = 2;

    let y = y + 3;

    {
        let y = y * 4;
        println!("El valor de 'y' es: {y}");
    }

    println!("El valor de 'y' es: {y}");

    // let espacios = "          ";
    let mut espacios = "          ";
    let espacios = espacios.len();
}
