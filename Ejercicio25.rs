/* Autor:  Orlando Urbano Trejo @Lando
 * Fecha:  25-08-2023
 * Correo: orlandourbanotrejo@gmail.com
 * Algoritmo para calcular el factorial de un numero. */

fn main() {
    let mut resultado = 1;
    // Entrada de datos
    println!("Ingresa un numero: ");
    let mut numero = String::new();
    std::io::stdin().read_line(&mut numero).expect("Error al leer la entrada");
    let numero: u32 = numero.trim().parse().expect("No es un número válido");
    for i in 1..=numero {
        resultado *= i;
    }
    println!("El factorial de {} es: {}", numero, resultado);
}

