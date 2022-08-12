/*
    Programa para practicar los Vectores en Rust.
    Ejercicio basado en una practica de una asignatura de porgramación.
*/
use std::io;
use std::{thread, time};

fn main() {

    let menu = ["1. Construir matiz","2. Introducir Matriz","3. Volcar Matriz", "4. Salir"];
    let mut repetir = true;

    let mut matriz:Vec<Vec<f32>> = Vec::new();

    while repetir == true {
        
        for frase in menu {

            println!("{}",frase);
        }

        let intro;
        
        println!("Por favor, introduzca la Opción que Desee: ");

         intro = uint_input_consola();

        if intro == 1 {
            println!("\nHas elegido Contruir Matriz");
            
            let nfil;
            println!("Por favor, introduzca El numero de FILAS que desea (u32): ");
            nfil = uint_input_consola();

            let ncol;
            println!("Por favor, introduzca El numero de COLUMNAS que desea (u32): ");
            ncol = uint_input_consola();

            if nfil == 0  || ncol == 0 {
                println!("Por favor introduzca Números validos ( > 0), La matriz no ha sido creada\n");
                let onesec = time::Duration::from_millis(1000);
                thread::sleep(onesec);
            }

            matriz = construir_matriz(nfil,ncol);

        }else if intro == 2 {
            introducir_datos_matriz(&mut matriz);

        }else if intro == 3 {
            println!("--------------------------");
            mostrar_matriz(&matriz);
            println!("--------------------------");

        }else if intro == 4 {
            println!("\n------- SALIENDO DEL PROGRAMA ---------");
            repetir = false;
            
        }else{
            println!("\n POR FAVOR, introduzca un numero del 1 al 4");
        }

    }
}

fn int_input_consola() -> f32{
    let mut num = String::new();
    io::stdin()
        .read_line(&mut num)
        .expect("\nERROR al leer el número");

    let num:f32 = match num.trim().parse() {
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

fn construir_matriz(n_filas:u32, n_columnas:u32) -> Vec<Vec<f32>> {
    let matriz = vec![vec![0.0;n_columnas.try_into().unwrap()];n_filas.try_into().unwrap()];
    matriz
}


fn introducir_datos_matriz(v:&mut Vec<Vec<f32>>){
    for i in 0..v.len()
    {
        for j in 0..v[i].len(){

            let num:f32;
            println!("\nIntroduzca el valor de v[{}][{}]: ",i,j);
            num = int_input_consola();

            v[i][j] = num;
        }
    }
}

fn mostrar_matriz( v:&Vec<Vec<f32>>) {
    for i in 0..v.len(){

        for j in 0..v[i].len(){

            print!(" {} ",v[i][j]);
        }
        print!("\n");
    }
}
