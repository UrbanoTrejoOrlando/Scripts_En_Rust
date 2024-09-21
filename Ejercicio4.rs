/* Autor:  Orlando Urbano Trejo @Lando
 * Fecha:  24-08-2023
 * Correo: orlandourbanotrejo@gmail.com
 * Algoritmo para calcular el tiempo que tarda una persona en llegar a un lugar */

fn main(){
    // Datos de entrada
    println!("Velocidad en (km/h): ");
    let mut km = String::new();
    std::io::stdin().read_line(&mut km).expect("Error al leer la velocidad"); 
    let metros: f32 = km.trim().parse().expect("No se pudo convertir");
    println!("Distancia a recorrer: ");
    let mut distancia = String::new();
    std::io::stdin().read_line(&mut distancia).expect("Error al leer la velocidad"); 
    let nueva_di: f32 = distancia.trim().parse().expect("No se pudo convertir");
    // Impresion de resultados
    println!("Tiempo: {}",(metros * nueva_di)/60.0);
}
