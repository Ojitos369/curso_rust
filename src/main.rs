fn temperatura() {
    // i8, i16 - enteros con signo
    // u8, u16 - enteros sin signo (positivos)
    // &str - string

    let temperatura_minima: i8 = -3;
    let temperatura_maxima: u8 = 32;
    let temperatura_promedio: f32 = (temperatura_minima + temperatura_maxima as i8) as f32/ 2.0;

    // println!("La termeratura minima en mi ciudad es {} y la maxima es {} y el promedio es {}", temperatura_minima, temperatura_maxima, temperatura_promedio);
    println!("La termeratura minima en mi ciudad es {temperatura_minima} y la maxima es {temperatura_maxima} y el promedio es {temperatura_promedio}");
}

fn ingreso_datos() {
    // &str -> para strings sencillos, mejor velocidad en procesamiento
    // String -> un poco mas complejo, con mas funciones

    println!("Ingrese su nombre: ");
    let mut nombre: String = String::new();
    std::io::stdin().read_line(&mut nombre).unwrap();

    // trim() -> Para eliminar espacio en blanco
    // to_string() -> Para convertir un &str a un String
    nombre = nombre.trim().to_string();
    
    println!("Ingrese su edad: ");
    let mut edad: String = String::new();
    std::io::stdin().read_line(&mut edad).unwrap();

    // trim() -> elimina los espacios en blanco
    // parse() -> convierte un string a un tipo de dato
    // unwrap() -> manejo de errores
    let edad_int: u8 = edad.trim().parse().unwrap();

    println!("Hola {nombre} que tiene {edad_int} años");
}


fn main() {
    println!("Ingresa la clase: ");

    let mut clase: String = String::new();
    std::io::stdin().read_line(&mut clase).unwrap();

    let clase_int: u8 = clase.trim().parse().unwrap();

    match clase_int {
        1 => temperatura(),
        2 => ingreso_datos(),
        _ => println!("Hola clase desconocida"),
    }
}
