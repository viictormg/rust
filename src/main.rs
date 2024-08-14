fn main() {
    println!("Hello, world!");

    // Numeros enteros sin signo
    let _ = 23u8;
    let _ = 123_u16;
    let _ = 123_u32;
    let _ = 123_u64;
    let _ = 123_u128;
    let _ = 123_usize;

   //numero enteros con signo
    let _ = 23i8;
    let _ = 123_i16;
    let _ = 123_i32;
    let _ = 123_i64;
    let _ = 123_i128;
    let _ = 123_isize;

    // Numeros flotantes
    let _ = 1.1f32;
    let _ = 1.1f64;
    let _ = 3.14e-10;

    // constante
    let _ =f32::MIN;
    let _ =f32::MAX;
    let _ =f64::INFINITY;
    let _ =f64::NEG_INFINITY;
    let _ =f64::NAN;

    // constantes matematicas
    let _ = std::f32::consts::PI;
    let _ = std::f32::consts::E;
    let _ = std::f64::consts::PI;
    let _ = std::f64::consts::E;

    for i in 0..10 {
        print!("valor de i = {}", i);
    }

    // para declarar una variable usamos la palabra reservada let y es similar a la asiganacion de go

    let x= 1;
    print!("valor de x = {}", x);

    let y: u16 = 2;

    let z = (x as u16) + y;
    
    print!("valor de z = {}", z);

    for i in 0..10 {
        print!("valor de i = {}", i);
    }

   
}
