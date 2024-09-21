/* Autor:  Orlando Urbano Trejo @Lando
 * Fecha:  25-08-2023
 * Correo: orlandourbanotrejo@gmail.com
 * Algoritmo que determina el precio del boleto de un viaje */
fn main(){
    // Entrada de datos
    println!("Kilometros a recorrer: ");
    let mut km = String::new();
    std::io::stdin().read_line(&mut km).expect("Error al leer la velocidad");
    let km: f32 = km.trim().parse().expect("No se pudo convertir");
    println!("El precio del boleto es: ${}",km * 20.50);
}

