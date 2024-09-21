/* Autor:  Orlando Urbano Trejo @Lando
 * Fecha:  25-08-2023
 * Correo: orlandourbanotrejo@gmail.com
 * Algoritmo que calcula el precio total del boleto de una persona, ingresando los datos por
 * teclado */
fn main() {
    // Datos de entrada
    println!("Precio por kilómetro: ");
    let mut precio_kilometro = String::new();
    std::io::stdin().read_line(&mut precio_kilometro).expect("Error al leer el precio");
    let precio_float: f32 = precio_kilometro.trim().parse().expect("No se pudo convertir el precio");

    println!("Cantidad de kilómetros a recorrer: ");
    let mut cantidad = String::new();
    std::io::stdin().read_line(&mut cantidad).expect("Error al leer la cantidad");
    let cantidad_float: f32 = cantidad.trim().parse().expect("No se pudo convertir la cantidad");
    // Operaciones
    let precio_boleto = precio_float * cantidad_float;
    println!("Precio del boleto = ${}", precio_boleto);
}

