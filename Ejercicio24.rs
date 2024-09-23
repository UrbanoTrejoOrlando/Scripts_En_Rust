/* Autor:  Orlando Urbano Trejo @Lando
 * Fecha:  22-07-2023
 * Correo: orlandourbanotrejo@gmail.com
 * Algoritmo que calcula la tercera parte de un numero */
fn main(){
    // Datos de entrada
    println!("Ingresa un numero: ");
    let mut numero = String::new();
    std::io::stdin().read_line(&mut numero).expect("Error al leer el numero");
    let numero: f32 = numero.trim().parse().expect("Error");
    // Operaciones
    let resultado1 = numero * 2.0;
    let resultado2 = resultado1 / 3.0;
    let resultado3 = resultado2  / 2.0;
    println!("La mitad de la tercera parte de {} es: {}",numero,resultado3);

}
