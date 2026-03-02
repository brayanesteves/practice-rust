fn main() {
    /**
     * [ENGLISH]
     * 
     * ==========
     * 
     * [SPANISH]
     * Ciclo 'while'.
     */
    let mut contador = 0;

    /**
     * [ENGLISH]
     * 
     * ==========
     * 
     * [SPANISH]
     * Ciclo 'while' que se ejecuta mientras `contador` sea menor que 5.
     */
    while contador < 5 {
        println!("El valor de 'contador' es: {}", contador);
        contador += 1; // Incrementa el 'contador' en 1.
    }

    println!("El ciclo ha terminado. Contador final: {}", contador);
}