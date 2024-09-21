/* Autor:  Orlando Urbano Trejo @Lando
 * Fecha:  25_08_2023
 * Correo: orlandourbanotrejo@gmail.com
 * Algoritmo para determinar el sueldo semanal de N trabajdores y aplicar descuentos dependiendo de
 * las horas se aplicara un descuento */
fn main() {
    // Datos en entrada
    println!("Numero de trabajadores: ");
    let mut trabajador = String::new();
    std::io::stdin().read_line(&mut trabajador).expect("Error al leer el número de trabajadores");
    let trabajador: usize = trabajador.trim().parse().expect("No se pudo convertir el número de trabajadores");

    let mut _sueldo: f64 = 0.0;
    let mut _nuevo_salario: f64 = 0.0;
    let mut _nombre = String::new();

    for i in 1..=trabajador {
        println!("Nombre del trabajador {}: ", i);
        std::io::stdin().read_line(&mut _nombre).expect("Error al leer el nombre"); 

        println!("Horas trabajadas: ");
        let mut horas = String::new();
        std::io::stdin().read_line(&mut horas).expect("Error al leer las horas trabajadas");
        let horas: f64 = horas.trim().parse().expect("No se pudo convertir las horas trabajadas");

        println!("Sueldo por hora: ");
        let mut salario = String::new();
        std::io::stdin().read_line(&mut salario).expect("Error al leer el sueldo por hora");
        let salario: f64 = salario.trim().parse().expect("No se pudo convertir el sueldo por hora");

        _sueldo = horas * salario;
        // Condiciones
        if salario >= 0.0 && salario <= 150.0 {
            _nuevo_salario = _sueldo * 0.5;
        } else if salario > 150.0 && salario <= 300.0 {
            _nuevo_salario = _sueldo * 0.7;
        } else if salario > 300.0 && salario <= 450.0 {
            _nuevo_salario = _sueldo * 0.9;
        } else {
            _nuevo_salario = 0.0;
        }

        let total = _sueldo - _nuevo_salario;

        println!("Trabajador: {}", _nombre.trim());
        println!("Salario final: {:.2}", total);
    }
}


