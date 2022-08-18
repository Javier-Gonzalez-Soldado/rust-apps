/*
    ?Programa que permita crear figuras geometricas regulares
    ?de cualquier tamaño que el cliente desee
    ?Y que permita calcular su area y perimetro
*/

//TODO: Pulir el codigo y considerar que el numero de lados tiene que se mayor o igual a 5 en los poligonos.

mod declaraciones;

use std::io;
use crate::declaraciones::Circulo;
use crate::declaraciones::Cuadrado;
use crate::declaraciones::Rectangulo;
use crate::declaraciones::Triangulo;
use crate::declaraciones::Poligono;
use crate::declaraciones::Area;
use crate::declaraciones::Perimetro;


fn main(){
    let menu = ["---------- MENU ----------","1. Crear un Cuadrado","2. Crear un Rectangulo","3. Crear un Circulo","4. Crear Triangulo","5. Crear Poligono","6. Salir"];
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
            triangulo();

        }else if intro == 5{
            poligono();
        }
        else if intro == 6{
            repetir = false;
            println!("\n------- SALIENDO DEL PROGRAMA ---------");
        }
        else{
            println!("POR FAVOR, introduzca un numero del 1 al 6\n");
        }

    }

}

fn float_input_consola() -> f64{
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
fn uint_input_consola() -> u64 {
    let mut num = String::new();
    io::stdin()
        .read_line(&mut num)
        .expect("\nERROR al leer el número");

    let num:u64 = match num.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,

    };
    num
}

fn cuadrado(){
    let lad;

    println!("Introduzca el tamaño del lado (m): ");
    lad = float_input_consola();

    let cuadrado = Cuadrado { lado:lad};

    println!("---------------------------------------");
    println!("El perimetro es: {} m",cuadrado.perimetro());
    println!("El area es: {} m2", cuadrado.area());
    println!("---------------------------------------\n");

}

fn rectangulo(){
    let b; let h ;
    
    println!("Introduzca el tamaño de la base (m): ");
    b = float_input_consola();

    println!("Introduzca el tamaño de la altura (m): ");
    h = float_input_consola();

    let rect = Rectangulo {
        base: b,
        altura: h
    };

    println!("---------------------------------------");
    println!("El perimetro es: {} m",rect.perimetro());
    println!("El area es: {} m2",rect.area());
    println!("---------------------------------------\n");

}

fn circulo(){
    let r ;

    println!("Introduzca el tamaño del radio (m): ");
    r = float_input_consola();

    let circ = Circulo { radio: r};
    println!("---------------------------------------");
    println!("El perimetro es: {} m",circ.perimetro());
    println!("El area es: {} m2",circ.area());
    println!("---------------------------------------\n");
}

fn triangulo(){
    let lad;
    println!("Introduzca el tamaño del lado (m): ");
    lad = float_input_consola();

    let triang = Triangulo { lado:lad};
    println!("---------------------------------------");
    println!("El perimetro es: {} m ",triang.perimetro());
    println!("El area es: {} m2 ",triang.area());
    println!("---------------------------------------\n");
}

fn poligono(){
    let lad; let nlad;
    println!("Introduzca el numero de lados que desea: ");
    nlad = uint_input_consola();

    println!("Introduzca el tamaño del lado (m): ");
    lad = float_input_consola();

    let poli = Poligono {
        nlados:nlad,
        lado:lad
    };

    println!("---------------------------------------");
    println!("El perimetro es: {} m ",poli.perimetro());
    println!("El area es: {} m2 ",poli.area());
    println!("---------------------------------------\n");

}