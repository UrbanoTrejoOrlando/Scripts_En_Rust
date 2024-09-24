/* Autor:  Orlando Urbano Trejo @Lando
 * Fecha:  25-08-2023
 * Correo: orlandourbanotrejo@gmail.com
 *
 * Algoritmo que calcula el descuento de 3 tipos de vestidos */
fn main() {
    println!("----VESTIDOS----\n1) Grande $5000\n2) Mediana $400\n3) Chica $300");
    let mut total: f32 = 0.0;
    // Entrada de datos
    println!("Elige una marca: ");
    let mut numero = String::new();
    std::io::stdin().read_line(&mut numero).expect("Error al leer la entrada");
    let marca: i32 = numero.trim().parse().expect("Error");
    // Condiciones
    if marca == 1 {
        println!("Numero de vestidos que deseas comprar: ");
        numero.clear();
        std::io::stdin().read_line(&mut numero).expect("Error a leer la entrada");
        let cantidad: f32 = numero.trim().parse().expect("Error");
        total = cantidad * 500.0;
    } 
    else if marca == 2 {
        println!("Numero de vestidos que deseas comprar: ");
        numero.clear();
        std::io::stdin().read_line(&mut numero).expect("Error a leer la entrada");
        let cantidad: f32 = numero.trim().parse().expect("Error");
        total = cantidad * 400.0;
    } 
    else if marca == 3 {
        println!("Numero de vestidos que deseas comprar: ");
        numero.clear();
        std::io::stdin().read_line(&mut numero).expect("Error a leer la entrada");
        let cantidad: f32 = numero.trim().parse().expect("Error");
        total = cantidad * 300.0;
    }
    println!("Monto total de la venta: ${}", total);
}

