/* Autor:  Orlando Urbano Trejo @Lando
 * Fecha:  25-07-2023
 * Correo: orlandourbanotrejo@gmail.com
 * Algoritmo que calcule el sueldo total de un empleado */

fn main(){
    // Entrada de datos
    println!("Horas trabajadas: ");
    let mut horas= String::new();
    std::io::stdin().read_line(&mut horas).expect("Error al leer las horas");
    let horas: f32 =  horas.trim().parse().expect("Error");
    println!("Sueldo por hora: ");
    let mut sueldo  =  String::new();
    std::io::stdin().read_line(&mut sueldo).expect("Error al leer el sueldo");
    let sueldo: f32 =  sueldo.trim().parse().expect("Error");
    println!("Sueldo total: ${}",horas * sueldo);

}


