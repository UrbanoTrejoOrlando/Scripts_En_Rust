/* Autor:  Orlando Urbano Trejo @Lando
 * Fecha:  25-08-2023
 * Correo: orlandourbanotrejo@gmail.com
 * Algoritmo que sume tres numero */
 
fn main(){
    // Entrada de datos
    println!("Numero 1: ");
    let mut numero1 = String::new();
    std::io::stdin().read_line(&mut numero1).expect("Error al leer el numero 1");
    let numero1: f32 = numero1.trim().parse().expect("Error");
    
    println!("Numero 2: ");
    let mut numero2 = String::new();
    std::io::stdin().read_line(&mut numero2).expect("Error al leer el numero 1");
    let numero2: f32 = numero2.trim().parse().expect("Error");
    
    println!("Numero 3: ");
    let mut numero3 = String::new();
    std::io::stdin().read_line(&mut numero3).expect("Error al leer el numero 1");
    let numero3: f32 = numero3.trim().parse().expect("Error");
    
    let suma = numero1 + numero2 + numero3;

    println!("Suma: {}",suma);

}


