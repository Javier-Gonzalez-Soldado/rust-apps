/*
    ?Programa que permita crear figuras geometricas regulares
    ?de cualquier tamaño que el cliente desee
    ?Y que permita calcular su area y perimetro
*/

//TODO: Pulir el codigo.

mod declaraciones;

use std::io;
use crate::declaraciones::Circulo;
use crate::declaraciones::Cuadrado;
use crate::declaraciones::Rectangulo;
use crate::declaraciones::Area;
use crate::declaraciones::Perimetro;


fn main(){
    let menu = ["1. Crear un Cuadrado","2. Crear un Rectangulo","3. Crear un Circulo", "4. Salir"];
    let mut repetir = true;

    while repetir == true {
        for frase in menu {
            println!("{}",frase);
        }

        let intro;
        
        println!("Por favor, introduzca la Opción que Desee: ");

        intro = uint_input_consola();

        if intro == 1 {
            cuadrado();

        }else if intro == 2{
            rectangulo();

        }else if intro == 3{
            circulo();
            
        }else if intro == 4{
            repetir = false;
            println!("\n------- SALIENDO DEL PROGRAMA ---------");
        }else{
            println!("\n POR FAVOR, introduzca un numero del 1 al 4");
        }

    }

}

fn int_input_consola() -> f64{
    let mut num = String::new();
    io::stdin()
        .read_line(&mut num)
        .expect("\nERROR al leer el número");

    let num:f64 = match num.trim().parse() {
        Ok(num) => num,
        Err(_) => 0.0,

    };
    num
}
fn uint_input_consola() -> u32 {
    let mut num = String::new();
    io::stdin()
        .read_line(&mut num)
        .expect("\nERROR al leer el número");

    let num:u32 = match num.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,

    };
    num
}

fn cuadrado(){
    let lad;

    println!("Introduzca el tamaño del lado (m): ");
    lad = int_input_consola();

    let cuadrado = Cuadrado { lado:lad};

    println!("El perimetro es: {} m",cuadrado.perimetro());
    println!("El area es: {} m2", cuadrado.area());

}

fn rectangulo(){
    let b; let h ;
    
    println!("Introduzca el tamaño de la base (m): ");
    b = int_input_consola();

    println!("Introduzca el tamaño de la altura (m): ");
    h = int_input_consola();

    let rect = Rectangulo {
        base: b,
        altura: h
    };

    println!("El perimetro es: {} m",rect.perimetro());
    println!("El area es: {} m2",rect.area());

}

fn circulo(){
    let r ;

    println!("Introduzca el tamaño del radio (m): ");
    r = int_input_consola();

    let circ = Circulo { radio: r};

    println!("El perimetro es: {} m",circ.perimetro());
    println!("El area es: {} m2",circ.area());
}