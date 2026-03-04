fn main() {
    println!("Hello, world!");

    {
        /**
         * [ENGLISH]
         * 
         * =========
         * 
         * [SPANISH]
         * Sigue estando dentro del 'Scope' variable 's'.
         */
        let s = "Hello";
    } // [ENGLISH] / [SPANISH] Cuando se cierra la llave sale el 'Scope' y 's' no es valida.

    let mut s = String::from("Hello");
    s.push_str(", world");
    println!("{s}");
}
