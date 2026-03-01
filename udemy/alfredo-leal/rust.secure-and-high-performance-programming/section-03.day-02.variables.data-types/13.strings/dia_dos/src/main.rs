fn main() {

    /**
     * [ENGLISH]
     * 
     * ==========
     * 
     * [SPANISH]
     * 
     * Shadowing.
     * Manejo de cadenas.
     */
    let cadena = "que tal";
    println!("{}", cadena);

    let mut saludo = String::from("Hola");
    saludo.push_str(", mundo"); // Agrega una cadena.
    saludo.push('☺️'); // Agrega un carácter.
    println!("{}", saludo); // "Hola, mundo!☺️"

    // str
    let saludo_str: &str = "Hola, mundo!"; // Cadena literal.
    println!("{}", saludo_str); // "Hola, mundo!"
}
