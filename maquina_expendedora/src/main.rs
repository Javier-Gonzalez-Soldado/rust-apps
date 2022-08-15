/*
 * Reto semanal #28 de la pagina web retosdeprogramacion.com realizado en Rust:
 
 * Simula el funcionamiento de una máquina expendedora creando una operación
 * que reciba dinero (array de monedas) y un número que indique la selección
 * del producto.
 * - El programa retornará el nombre del producto y un array con el dinero
 *   de vuelta (con el menor número de monedas).
 * - Si el dinero es insuficiente o el número de producto no existe,
 *   deberá indicarse con un mensaje y retornar todas las monedas.
 * - Si no hay dinero de vuelta, el array se retornará vacío.
 * - Para que resulte más simple, trabajaremos en céntimos con monedas
 *   de 5, 10, 50, 100 y 200.
 * - Debemos controlar que las monedas enviadas estén dentro de las soportadas.
 */

use std::{collections::HashMap};


 fn main(){
    comprar(1,[5,10,50]);
    comprar(3,[50,10,100]);
    comprar(34,[200,50,10]);
    comprar(2,[5,10,5]);
 }

 fn comprar(id:u8, monedas:[i32;3]){
    
   let mut productos = HashMap::new();
   productos.insert(1,("Coca Cola",50));
   productos.insert(2,("Red Bull",80));
   productos.insert(34,("Fanta",100));

   for prod in productos {
      if prod.0 == id {
         if comprobar_monedas(prod.1.1,monedas){
            let cambio = calcular_cambio(prod.1.1 , monedas);
            println!("Ha comprado: {} ",prod.1.0);
            println!("El cambio devuelto: {:?}",cambio);
            println!("\n");
            return ;
         }else{
            println!("\nERROR, numero de monedas insuficiente");
            println!("\n");
            return;
         }

      }else{ 
         continue;
      }
   }
   println!("\nERROR, numero de id invalido");
   println!("\n");
 


 }

 fn comprobar_monedas(precio:i32,monedas:[i32;3]) -> bool{
   let mut suma = 0;
   for mon in monedas {
      suma += mon;
   }
   if suma < precio{
      return false;
   }else{
      return true;
   }
 }

 fn calcular_cambio(precio:i32,monedas:[i32;3]) -> Vec<i32>{
   let mut suma = 0;
   let mut cantidad;
   for mon in monedas {
      suma += mon;
   }
   let mut cambio = suma - precio;

   let mut mon_camb = Vec::new();
   
   while cambio > 0 {
      if cambio >= 200{
         cantidad = cambio / 200;
         cambio -= 200 * cantidad;
         while cantidad > 0 {
            mon_camb.push(200);
            cantidad-=1;
         }   
      }
      else if cambio >= 100 {
          cantidad = cambio / 100;
          cambio -= 100 * cantidad;
          while cantidad > 0 {
            mon_camb.push(100);
            cantidad-=1;
         } 

      }
      else if cambio >= 50 {
          cantidad = cambio / 50;
          cambio -= 50 * cantidad;
          while cantidad > 0 {
            mon_camb.push(50);
            cantidad-=1;
         } 
      }
      else if cambio >= 10 {
         cantidad = cambio / 10;
          cambio -= 10 * cantidad;
          while cantidad > 0 {
            mon_camb.push(10);
            cantidad-=1;
         } 
      }
      else if cambio >= 5 {
         cantidad = cambio / 5;
         cambio -= 5 * cantidad;
         while cantidad > 0 {
            mon_camb.push(5);
            cantidad-=1;
         } 
      }

   }
   mon_camb
}

 