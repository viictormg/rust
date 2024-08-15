

fn main() {
    prueba_imutabilidad();  
    shadowing(); 
}

fn prueba_imutabilidad(){
    let x: i32 = 1;
    println!("el valor x es {}", x);
    let x = 6;

    println!("el valor de x ahora es {}", x)

}

fn shadowing() {
    let x: i32 = 5;
    let x: i32 = x + 23;

    let y : i128 = 99999999999999999999999999999999999999;
    let y : i128 = y +1;
    println!("este es el valor reasignado {}", x);
    println!("este es el valor reasignado {}", y);
}

