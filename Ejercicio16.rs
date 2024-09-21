/* Autor:  Orlando Urbano Trejo @Lando
 * Fecha:  25-07-2023
 * Correo: orlandourbanotrejo@gmail.com
 * Algoritmo que calcula el presupuesto anual en tres areas de un hospital */

fn main(){
    // Entrada de datos
    println!("Presupuesto de cada año: ");
    let mut anual = String::new();
    std::io::stdin().read_line(&mut anual).expect("Error al leer el valr");
    let anual : f32 = anual.trim().parse().expect("No se pudo convertir");
    // Impresion de resultados
    println!("Presupuesto Urgencias: ${}", anual * 0.47);
    println!("Presupuesto Pediatria: ${}", anual * 0.42);
    println!("Presupuesto Traumatologia: ${}", anual * 0.21);
}
