fn main() {

    /**
     * [ENGLISH]
     * 
     * ==========
     * 
     * [SPANISH]
     * 
     * Shadowing.
     * Matrices.
     */
    let matriz = [10, 20, 30, 40, 50];

    let days_of_weeks = ["Lunes", "Martes", "Miércoles", "Jueves", "Viernes", "Sábado", "Domingo"];

    let matriz: [i32; 5] = [10, 20, 30, 40, 50];

    /**
     * [ENGLISH]
     * 
     * ==========
     * 
     * [SPANISH]
     * Accediendo a los elementos.
     */
    let first  = matriz[0];
    let second = matriz[1];
    println!("El valor de 'second' es: {second}");
}
