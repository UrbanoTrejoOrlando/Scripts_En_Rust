/* Autor:  Orlando Urbano Trejo @Lando
 * Fecha:  25-08-2023
 * Correo: orlandourbanotrejo@gmail.com
 * Algoritmo que calcula la cantidad de euros a monedas y viceversa */

fn main(){
    println!("------MENU DE CONVERSIONES------\n1) Euros\n2) Dolares");
    // Entrada de datos
    println!("Elige una opcion: ");
    let mut opcion = String::new();
    std::io::stdin().read_line(&mut opcion).expect("Error no se pudo leer el dato");
    let opcion: i8 = opcion.trim().parse().expect("Error");
    // Condiciones
    if opcion == 1 {
        println!("Dinero: ");
        let mut dinero = String::new();
        std::io::stdin().read_line(&mut dinero).expect("Error no se pudo leer el dato");
        let dinero: f32 = dinero.trim().parse().expect("Error");
        let resultado = dinero / 16.89;
        println!("Dinero = {} dolares",resultado);
    }
    else if opcion == 2 {
        println!("Dinero: ");
        let mut dinero = String::new();
        std::io::stdin().read_line(&mut dinero).expect("Error no se pudo leer el dato");
        let dinero: f32 = dinero.trim().parse().expect("Error");
        let resultado = dinero / 0.053;
        println!("Dinero = ${} euros",resultado);

    }
    else{
        println!("Opcion no valida");
    }


}
