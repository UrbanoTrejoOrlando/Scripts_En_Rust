/* Autor:  Orlando Urbano Trejo @Lando
 * Fecha:  22-08-2023
 * Correo: orlandourbanotrejo@gmail.com
 * Algoritmo que calcula los intereses que tiene una persona con el banco */
fn main() {
    let mut prestamo: f64 = 1000.0;
    // Entrada de datos
    println!("Año en el que solicitaste el prestamo: ");
    let mut anio = String::new();
    std::io::stdin().read_line(&mut anio).expect("Error al leer el año");
    let anio: i32 = anio.trim().parse().expect("No se pudo convertir el año");

    println!("Año actual: ");
    let mut actual = String::new();
    std::io::stdin().read_line(&mut actual).expect("Error al leer el año actual");
    let actual: i32 = actual.trim().parse().expect("No se pudo convertir el año actual");
    // Ciclo for
    for i in anio..=actual {
        prestamo = prestamo + (prestamo * 0.27);
        println!("El interés de {} es de: ${:.2}", i, prestamo);
    }
}

