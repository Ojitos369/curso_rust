fn pildoras() {
    println!("Que pildora quieres tomar (roja, azul)? ");
    let mut pildora: String = String::new();

    std::io::stdin().read_line(&mut pildora).unwrap();
    pildora = pildora.trim().to_string();

    if pildora == "roja" {
        println!("Bienvenido a la matrix");
    } else if pildora == "azul" {
        println!("Has despertado en tu cama sin saber nada, como un dia normal");
    } else {
        println!("Ups... has quedado glitcheado por toda la eternidad..... .... ... .. .");
    }

}

fn condicionales() {
    println!("Ingresa tu edad: ");
    let mut edad_str: String = String::new();

    std::io::stdin().read_line(&mut edad_str).unwrap();

    let edad: u8 = edad_str.trim().parse().unwrap();

    if edad < 12 {
        println!("Eres un niño");
    } else if edad < 18 {
        println!("Eres un adolescente");
    } else {
        println!("Eres un adulto");
        if edad > 65 {
            println!("Eres un adulto mayor");
        } else if edad == 27 {
            println!("Tienes mi edad, bienvenido");
        } else {
            println!("Bienvenido");
        }
    }
}

fn main() {
    println!("Clase: 1 \nReto: 2");
    let mut opcion: String = String::new();
    std::io::stdin().read_line(&mut opcion).unwrap();

    opcion = opcion.trim().to_string();

    if opcion == "1" {
        condicionales();
    } else if opcion == "2" {
        pildoras();
    } else {
        println!("Opcion no valida");
    }

}
