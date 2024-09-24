/* Autor:  Orlando Urbano Trejo @Lando
 * Fecha:  25-08-2023
 * Correo: orlandourbanotrejo@gmail.com
 * Algoritmo que almacena calificaciones de alumnos y calcula su promedio */

fn main() {
    let mut calificacion = [0; 6];
    let mut suma: f64 = 0.0;
    // Entrada de datos
    println!("Materias: ");
    let mut materia = String::new();
    std::io::stdin().read_line(&mut materia).expect("Error al leer el nombre de la materia");
    let materia = materia.trim();

    for i in 1..=5 {
        println!("Calificacion en la unidad {}: ", i);
        let mut calif = String::new();
        std::io::stdin().read_line(&mut calif).expect("Error al leer la calificación");
        calificacion[i] = calif.trim().parse().expect("No se pudo convertir la calificación");
        suma += calificacion[i] as f64;
    }

    println!("Asignatura: {}", materia);
    println!("Unidad 1\tUnidad 2\tUnidad 3\tUnidad 4\tUnidad 5");

    for i in 1..=5 {
        print!("{}\t\t", calificacion[i]);
    }
    // Operaciones
    let promedio = suma / 5.0;
    // Impresion de resultados
    println!("\nPromedio: {:.2}", promedio);
}

