fn main() {
    /**
     * [ENGLISH]
     * 
     * ==========
     * 
     * [SPANISH]
     * Loop.
     * Este 'loop' se va a detener, hasta que la memoria RAM del ordenador se agote. Para detenerlo es teclear simultaneamente 'Ctrl' + 'C'.
     */
    // loop {
    //     println!("Bucle infinito.");
    // }

    /**
     * [ENGLISH]
     * 
     * ==========
     * 
     * [SPANISH]
     * Ejemplo 1.
     */
    let mut contador_1 = 0;

    let resultado = loop {
        contador_1 += 1;

        if contador_1 == 10 {
            break contador_1 * 2;
        }
    };
    println!("[Ejemplo N°1] El resultado es {resultado}");

    /**
     * [ENGLISH]
     * 
     * ==========
     * 
     * [SPANISH]
     * Ejemplo 2.
     */
    let mut contador_2 = 0;
    'counting_up: loop {
        println!("contador = {contador_2}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if contador_2 == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        contador_2 += 1;
    }
    println!("[Ejemplo N°2] Fin contador = {contador_2}");
}