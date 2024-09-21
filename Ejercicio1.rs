/* Autor:  Orlando Urbano Trejo @Lando
 * Fecha:  25-08-2023
 * Correo: orlandourbanotrejo@gmail.com
 * Algoritmo que determina a que letra perteneces segun la calificacion obtenida.*/

fn main() {
    // Dato de entrada
    println!("Ingrese una calificación (0-10): ");
    let mut calificacion = String::new();
    std::io::stdin().read_line(&mut calificacion).expect("Error al leer la entrada");

    let calificacion1_int: u8 = calificacion.trim().parse().expect("Error al analizar la calificación");
    // Evaluacion de datos
    match calificacion1_int {
        1..=5 => println!("Tu calificación es F"),
        6..=7 => println!("Tu calificación es D"),
        8 => println!("Tu calificación es C"),
        9 => println!("Tu calificación es B"),
        10 => println!("Tu calificación es A"),
        _ => println!("Calificación fuera de rango"),
    }
}

