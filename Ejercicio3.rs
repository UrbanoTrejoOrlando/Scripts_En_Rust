/* Autor:  Orlando Urbano Trejo @Lando
 * Fecha:  25-08-2023
 * Correo: orlandourbanotrejo@gmail.com
 * Algoritmo que calcule el total de una caja registradora */
fn main() {
    let mut resultado: i32 = 0;
    let dinero: [f32; 12] = [1000.0, 500.0, 200.0, 100.0, 50.0, 20.0, 5.0, 2.0, 1.0, 0.50, 0.20, 0.10];
    for i in 0..12 {
        // Datos de entrada
        println!("Cantidad de dinero que tiene: {:.2} pesos", dinero[i]);
        let mut convertidor = String::new();
        std::io::stdin().read_line(&mut convertidor).expect("Error al leer la cantidad");
        let convertidor: i32 = convertidor.trim().parse().expect("No se pudo convertir la cantidad");
        // Operaciones
        resultado += convertidor * (dinero[i] * 100.0) as i32;
        println!("Total almacenado: ${:.2} pesos", resultado as f32 / 100.0);
    }
}


