/* Autor:  Orlando Urbano Trejo @Lando
 * Fecha:  25-08-2023
 * Correo: orlandourbanotrejo@gmail.com
 * Algoritmo que imprima un arbol de navidad */

use std::io;

fn main() {
    // Datos de entrada
    println!("Altura del arbol: ");
    let mut altura = String::new();
    io::stdin().read_line(&mut altura).expect("Error al leer la altura");
    let altura: i32 = altura.trim().parse().expect("No se pudo convertir la altura");
    // Imprimir la parte superior del arbol
    for i in 1..=altura {
        let espacios = altura - i;
        // Imprimir espacios en blanco
        for _j in 1..=espacios {
            print!(" ");
        }
        // Imprimir asteriscos
        for _j in 1..=(2 * i - 1) {
            print!("*");
        }

        println!();
    }

    let tronco = altura - 1;
    // Imprimir tronco del arbol
    for _i in 1..=2 {
        for _j in 1..=tronco {
            print!(" ");
        }
        println!("*");
    }
}

