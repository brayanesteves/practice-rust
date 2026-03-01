fn main() {
    /**
     * [ENGLISH]
     * 
     * ==========
     * 
     * [SPANISH]
     * Expresión 'if' 'else'.
     */
    let numero = 1;

    if numero < 3 {
        println!("Condición verdadera.");
    } else {
        println!("Condición falsa.");
    }

    let numero = 10;

    if numero % 4 == 0 {
        println!("Número divisible por 4.");
    } else if numero % 3 == 0 {
        println!("Número divisible por 3.");
    } else if numero % 2 == 0 {
        println!("Número divisible por 2.");
    } else {
        println!("Número no es divisible por 4, 3, ó 2.");
    }

    let condicion = true;
    let numero = if condicion { 1 } else { 2 };
    println!("El valor de 'numero' es: {numero}");
}