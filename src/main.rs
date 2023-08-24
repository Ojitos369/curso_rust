// i8, i16 - enteros con signo
// u8, u16 - enteros sin signo (positivos)
// &str - string
fn temperatura() {

    let temperatura_minima: i8 = -3;
    let temperatura_maxima: u8 = 32;
    let temperatura_promedio: f32 = (temperatura_minima + temperatura_maxima as i8) as f32/ 2.0;

    // print!("La termeratura minima en mi ciudad es {} y la maxima es {} y el promedio es {}", temperatura_minima, temperatura_maxima, temperatura_promedio);
    print!("La termeratura minima en mi ciudad es {temperatura_minima} y la maxima es {temperatura_maxima} y el promedio es {temperatura_promedio}");
}


fn main() {
    temperatura();
}
