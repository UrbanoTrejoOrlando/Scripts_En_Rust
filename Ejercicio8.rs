/* Autor:  Orlando Urbano Trejo @Lando
 * Fecha:  25-08-2023
 * Correo: orlandourbanotrejo@gmail.com
 * Algoritmo para determinar el pago que debe realizar una persona por el consumo de agua */
fn main(){
    // Datos de entrada
    println!("Ingresa el precio por metro: ");
    let mut precio = String::new();
    std::io::stdin().read_line(&mut precio).expect("Error al leer la velocidad");
    let precio: f32 = precio.trim().parse().expect("No se pudo convertir");
    println!("Metros trabajados: ");
    let mut metros = String::new();
    std::io::stdin().read_line(&mut metros).expect("Error al leer el precio");
    let metros: f32 = metros.trim().parse().expect("No se pudo convertir el precio");
    // Impresion de resultados
    println!("Pago: ${}",precio * metros );
}
