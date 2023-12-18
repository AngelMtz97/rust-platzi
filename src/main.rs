fn main() {
    println!("Por favor introduce tu nombre: ");

    let mut nombre: String = String::new(); // Formas mas complejas, trim
    std::io::stdin().read_line(&mut nombre).unwrap();

    nombre = nombre.trim().to_string();

    // Obtener la edad de la consola
    let mut edad : String = String::new();
    println!("Por favor introduce tu edad: ");
    std::io::stdin().read_line(&mut edad).unwrap();

    // Convertir esa edad a un numero
    let edad_int : u8 = edad.trim().parse().unwrap();

    println!("Hola bienvenido {} de {} años", nombre, edad_int);
}
