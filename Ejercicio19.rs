/* Autor:  Orlando Urbano Trejo @Lando
 * Fecha:  25-08-2023
 * Correo: orlandourbanotrejo@gmail.com
 * Algoritmo que calcule el sueldo total de un empleado */

fn main(){
    // Entrada de datos
    println!("Ingrese su nombre: ");
    let mut nombre: String = String::new();
    std::io::stdin().read_line(&mut nombre).unwrap();
    nombre = nombre.trim().to_string();
    println!("Salario: ");
    let mut salario = String::new();
    std::io::stdin().read_line(&mut salario).expect("Error al leer el medicamento");
    let salario: f32 = salario.trim().parse().expect("Error");
    
    println!("Empleado: {} su salario es de: ${}",nombre,salario - (salario * 0.20));
}

