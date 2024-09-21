/* Autor:  Orlando Urbano Trejo @Lando
 * Fecha:  25-08-2023
 * Correo: orlandourbanotrejo@gmail.com
 * Algoritmo que calcula el saldo de sus clientes para no generar intereses y dependiendo del año aplicar descuentos */
fn main() {
    let mut _pago_actual: f64 = 0.0;
    let mut _saldo_minimo: f64 = 0.0;
    let mut _pago_interes: f64 = 0.0;
    // Entrada de datos 
    println!("Numero de clientes: ");
    let mut _cliente = String::new();
    std::io::stdin().read_line(&mut _cliente).expect("Error al leer el número de clientes");
    let _cliente: usize = _cliente.trim().parse().expect("No se pudo convertir el número de clientes");

    for _i in 1..=_cliente {
        // Almacenar datos de entrada
        println!("Nombre del cliente: ");
        let mut _nombre = String::new();
        std::io::stdin().read_line(&mut _nombre).expect("Error al leer el nombre");

        println!("Saldo anterior: ");
        let mut _saldo_anterior = String::new();
        std::io::stdin().read_line(&mut _saldo_anterior).expect("Error al leer el saldo anterior");
        let _saldo_anterior: f64 = _saldo_anterior.trim().parse().expect("No se pudo convertir el saldo anterior");

        println!("Ultimo deposito: ");
        let mut _deposito = String::new();
        std::io::stdin().read_line(&mut _deposito).expect("Error al leer el último deposito");
        let _deposito: f64 = _deposito.trim().parse().expect("No se pudo convertir el último deposito");

        println!("Monto por ventas: ");
        let mut _monto_compras = String::new();
        std::io::stdin().read_line(&mut _monto_compras).expect("Error al leer el monto por ventas");
        let _monto_compras: f64 = _monto_compras.trim().parse().expect("No se pudo convertir el monto por ventas");

        println!("Saldo actual: ");
        let mut _saldo_actual = String::new();
        std::io::stdin().read_line(&mut _saldo_actual).expect("Error al leer el saldo actual");
        let _saldo_actual: f64 = _saldo_actual.trim().parse().expect("No se pudo convertir el saldo actual");
        // Operaciones
        _pago_actual = (_saldo_actual * 0.12) + 200.0;
        _saldo_minimo = _saldo_actual * 0.15;
        _pago_interes = _saldo_actual * 0.85;
        // Impresion de resultados
        println!("Tu saldo actual {} es de: ${:.2}", _nombre.trim(), _pago_actual);
        println!("Tu pago minimo {} es de: ${:.2}", _nombre.trim(), _saldo_minimo);
        println!("El pago para no generar intereses {} es de: ${:.2}", _nombre.trim(), _pago_interes);
    }
}

