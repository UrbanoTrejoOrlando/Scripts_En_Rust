/* Orlando Urbano Trejo @Lando
 * Fecha: 25-08-2023
 * Correo: orlandourbanotrejo@gmail.com 
 * Algoritmo para determinar si eres mayor de edad. */
fn main(){
    // Entrada de datos
    println!("Edad: ");
    let mut edad = String::new();
    std::io::stdin().read_line(&mut edad).expect("Error al leer la edad");
    let edad: i8 = edad.trim().parse().expect("Error");
    // Condiciones
    if edad >= 18 {
        println!("Eres mayor de edad");
    }
    else{
        println!("Eres menor de edad");
    } 
}


