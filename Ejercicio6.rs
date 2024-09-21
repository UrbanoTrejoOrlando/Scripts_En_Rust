/* Autor:  Orlando Urbano Trejo @Lando
 * Fecha:  25-07-2023
 * Correo: orlandourbanotrejo@gmail.com
 * Algoritmo para determinar el costo que tendra realizar una llamada telefonica */
 
fn main(){
    // Datos de entrada
    println!("Tiempo de llamada en minutos: ");
    let mut tiempo = String::new();
    std::io::stdin().read_line(&mut tiempo).expect("Error");
    let tiempo_float: f32 = tiempo.trim().parse().expect("Error");  
     println!("Costo por minuto en pesos: ");
    let mut costo = String::new();
    std::io::stdin().read_line(&mut costo).expect("Error al leer al numero");
    let costo_float: f32 = costo.trim().parse().expect("No se pudo converitr el dato");
    // Impresion de datos
    println!("Costo de la llamada: ${}",tiempo_float * costo_float);
}
