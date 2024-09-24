/* Autor:  Orlando Urbano Trejo @Lando
 * Fecha:  25-08-2023
 * Correo: orlandourbanotrejo@gmail.com
 * Algoritmo que calcula la calficacion de N alumnos y calcula el promedio general */

fn main(){
    // Entrada de datos 
    println!("Numero de alumnos en su salon:  ");
    let mut alumno = String::new();
    std::io::stdin().read_line(&mut alumno).expect("Error al ingresar los alumnos");
    let alumno: i64 = alumno.trim().parse().expect("Error");

    let mut suma: i64 = 0;

    for i in 1..=alumno{
        println!("Edad del alumno {}:",i);
        let mut edad = String::new();
        std::io::stdin().read_line(&mut edad).expect("Error al ingresar la edad");
        let edad: i64 = edad.trim().parse().expect("Error");
        
        suma += edad;
    }
    // Operaciones
    let promedio = suma / alumno;
    println!("El promedio de los alumnos es de: {}",promedio);
}
