/* Autor:  Orlando Urbano Trejo @Lando
 * Fecha:  25-08-20i23
 * Correo: orlandourbanotrejo@gmail.com
 * Algoritmo para leer calificaciones de N alumnos y calcular el numero de aprobados y reprobados */
fn main(){
    // Datos de entrada
    println!("Cantidad de alumnos: ");
    let mut alumnos = String::new();
    std::io::stdin().read_line(&mut alumnos).expect("Error al leer la cantidad de alumnos");
    let alumnos: u8 = alumnos.trim().parse().expect("No se pudo convertir la cantidad de alumnos");
    for i in 1..=alumnos {
        // Recabar calificacion de alumnos
        println!("Calificación del alumno {}: ", i);
        let mut calificacion = String::new();
        std::io::stdin().read_line(&mut calificacion).expect("Error al leer la calificación");
        let calificacion: f64 = calificacion.trim().parse().expect("No se pudo convertir la calificación");
        // Condiciones
        if calificacion >= 70.0 {
            println!("Aprobado");
        } else {
            println!("Reprobado");
        }
    }
}

