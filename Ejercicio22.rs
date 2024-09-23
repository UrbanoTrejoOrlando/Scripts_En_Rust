/* Autor:  Orlando Urbano Trejo @Lando
 * Fecha:  25-08-2023
 * Correo: orlandourbanotrejo@gmail.com
 * Algoritmo que calcula las comisiones de un empleado dependiendo de las ventas realizadas  */

fn main(){
    // Entrada de datos
    println!("Sueldo fijo: ");
    let mut sueldo = String::new();
    std::io::stdin().read_line(&mut sueldo).expect("Error al leer el suelco");
    let sueldo: f32 = sueldo.trim().parse().expect("Error");
    println!("Ventas realizadas: ");
    let mut ventas = String::new();
    std::io::stdin().read_line(&mut ventas).expect("Error al leer las ventas");
    let ventas: i8 = ventas.trim().parse().expect("Error");

    for i in 0..ventas {
        println!("Precio de la venta {}: ",(i+1));
        let mut ventas1 = String::new();
        std::io::stdin().read_line(&mut ventas1).expect("Error al leer las ventas");
        let ventas1: f32 = ventas1.trim().parse().expect("Error");
        let comision = ventas1 * 0.10;
    }
    let total = comision + sueldo;
    println!("Comision: ${}",comision);
    println!("Sueldo: ${}",sueldo);

}


