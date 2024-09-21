/* Autor:  Orlando Urbano Trejo @Lando
 * Fecha:  25-08-2023
 * Correo: orlandourbanotrejo@gmail.com
 * Algoritmo que calcula la multiplicacion de dos numeros */

fn main() {
    // Entrada de datos
    println!("Ingrese un numero: ");
    let mut numero1: String = String::new();
    std::io::stdin().read_line(&mut numero1).unwrap();
    let numero1_int: f32 = numero1.trim().parse().unwrap();

    println!("Ingrese otro numero: ");
    let mut numero2: String = String::new();
    std::io::stdin().read_line(&mut numero2).unwrap();
    let numero2_int: f32 = numero2.trim().parse().unwrap();
    //  Impresion de resultados
    println!("La multiplicacion  de {} * {} es: {}", numero1_int,numero2_int,numero1_int*numero2_int);
}


