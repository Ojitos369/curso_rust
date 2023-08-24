// https://github.com/Ojitos369/curso_rust

fn reto() {
    println!("En esta clase no hubo reto :'c");
}

fn clase() {
    // Si no se pone el tipo de dato Rust le asigna automaticamente dependiendo del contexto
    // Es buena practica asignar el tipo pero si es obvio no es necesario
    let numero_1 = 123; 
    let numero_2 = 321;
    let suma = numero_1 + numero_2;

    loop {
        let mut respuesta_str = String::new();
        println!("Cuanto es la suma de {} + {}?", numero_1, numero_2);
        std::io::stdin().read_line(&mut respuesta_str).unwrap();
        let respuesta: i16 = respuesta_str.trim().parse().unwrap();
        if respuesta == suma {
            println!("La respuesta es correta");
            break;
        } else {
            println!("La respuesta es incorrecta. Intenta nuevamente.");
        }
    }

}

fn main() {
    println!("1: Clase \n2: Reto\nQue quieres ejecutar?");
    let mut opcion: String = String::new();
    std::io::stdin().read_line(&mut opcion).unwrap();

    opcion = opcion.trim().to_string();

    if opcion == "1" {
        clase();
    } else if opcion == "2" {
        reto();
    } else {
        println!("Opcion no valida");
    }
}
