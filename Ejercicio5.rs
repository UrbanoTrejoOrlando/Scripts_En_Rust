/* Autor:  Orlando Urbano Trejo @Lando
 * Fecha:  25-08-2023 
 * Correo: orlandourbanotrejo@gmail.com
 * Algoritmo que calcula la cantidad de ventas que realiza un vendedor y de acuerdo a esas ventas, realizar un descuento */

fn main() {
    // Datos de entrada
    println!("Número de ventas: ");
    let mut ventas = String::new();
    std::io::stdin().read_line(&mut ventas).expect("Error al leer el número de ventas");
    let ventas: usize = ventas.trim().parse().expect("No se pudo convertir el número de ventas");

    let mut _total_final: f64 = 0.0; 
    let mut _total: f64 = 0.0; 

    for i in 0..ventas {
        println!("Precio de la venta {}", i + 1);
        let mut precio = String::new();
        std::io::stdin().read_line(&mut precio).expect("Error al leer el precio");
        let precio: f64 = precio.trim().parse().expect("No se pudo convertir el precio");

        _total_final += precio;
        // Condiciones
        if precio < 1000.0 {
            _total += precio;
        } else if precio >= 1000.0 && precio < 2000.0 {
            _total += precio;
        }
    }
    // Impresion de resultados
    println!("Total final = ${:.2}", _total_final);
}

