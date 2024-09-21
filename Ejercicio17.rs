/* Autor:  Orlando Urbano Trejo @Lando
 * Fecha:  12-07-2023
 * Correo: orlandourbanotrejo@gmail.com
 * Algoritmo que calcula la cantidad total de dinero que tienes */
fn main(){
    let mut resultado: i32 = 0;
    let cajero: [f32; 11] = [1000.0, 500.0, 200.0, 100.0, 50.0, 20.0, 10.0, 5.0, 2.0, 1.0, 0.50];
    for i in 0..12{
        // Entrada de datos
        println!("Cantidad de dinero que tiene: {:.2} pesos", cajero[i]);
        let mut convertidor = String::new();
        std::io::stdin().read_line(&mut convertidor).expect("Error no puede leer la cantidad");
        let convertidor: i32 = convertidor.trim().parse().expect("No se pudo convertir la cantidad");
        resultado += convertidor * (cajero[i] * 100.0) as i32;
        // Impresion de datos
        println!("El dinero total almacenado: ${:.2} pesos", resultado as f32 / 100.0)
        }
}
