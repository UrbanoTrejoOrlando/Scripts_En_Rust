/* Orlando Urbano Trejo @Lando
 * Fecha: 25-08-2023
 * Correo: orlandourbanotrejo@gmail.com
 * Algoritmo que calcula la edad de una persona en meses. */

fn main(){
    // Entrada de datos
    println!("Edad en años: ");
    let mut edad = String::new();
    std::io::stdin().read_line(&mut edad).expect("Error al leer la edad");
    let edad: i32 = edad.trim().parse().expect("Error");
    
    println!("Cuantos meses: ");
    let mut meses = String::new();
    std::io::stdin().read_line(&mut meses).expect("Error al leer el mes");
    let meses: i32 = meses.trim().parse().expect("Error");
    // Operaciones
    let resultado = (edad * 12) + meses;
    println!("Edad: {} meses",resultado);


}
