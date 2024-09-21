/* Autor:  Orlando Urbano Trejo @Lando
 * Fecha:  25-08-2023
 * Correo: orlandourbanotrejo@gmail.com
 * Algoritmo para calcular el descuento de un medicamento  */
fn main(){
    // Entrada de datos
    println!("Ingrese su nombre: ");
    let mut nombre: String = String::new();
    std::io::stdin().read_line(&mut nombre).unwrap();
    nombre = nombre.trim().to_string();
    println!("Precio del medicamento: ");
    let mut precio = String::new();
    std::io::stdin().read_line(&mut precio).expect("Error al leer el medicamento");
    let precio: f32 = precio.trim().parse().expect("Error");
    // Operaciones
    let total = precio - (precio * 0.35);
    println!("Cliente: {} el total a pagar es de: ${}",nombre,total);

    
}

