fn main() {
    println!("Por favor introduce tu nombre: ");

    let mut nombre: String = String::new(); // Formas mas complejas, trim
    let mut apellido: &str; // Sirve mara mostrar texto, algo más sencillo

    std::io::stdin().read_line(&mut nombre).unwrap();

    println!("Hola bienvenido {}", nombre);
}
