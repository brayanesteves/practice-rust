fn main() {
    /**
     * [ENGLISH]
     * 
     * ==========
     * 
     * [SPANISH]
     * Ciclo 'for'.
     * Un rango que va desde '0' hasta '4' (Sin incluir el '5').
     */
    for numero in 0..5 {
        println!("El valor de actual es: {}", numero);
    }

    println!("[Ejemplo 1] El ciclo 'for' ha terminado.");

    /**
     * [ENGLISH]
     * 
     * ==========
     * 
     * [SPANISH]
     * Ejemplo 2.
     */
    let numeros = vec![10, 20, 30, 40, 50];

    for numero in numeros {
        println!("El número es: {}", numero);
    }

    println!("[Ejemplo 2] El ciclo 'for' sobre el vector ha terminado.");
}