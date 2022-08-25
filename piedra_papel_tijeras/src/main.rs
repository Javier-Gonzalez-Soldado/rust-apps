/*
 !Reto semanal #25.
 * Crea un programa que calcule quien gana más partidas al piedra,
 * papel, tijera.
 * - El resultado puede ser: "Player 1", "Player 2", "Tie" (empate)
 * - La función recibe un listado que contiene pares, representando cada jugada.
 * - El par puede contener combinaciones de "R" (piedra), "P" (papel)
 *   o "S" (tijera).
 * - Ejemplo. Entrada: [("R","S"), ("S","R"), ("P","S")]. Resultado: "Player 2".
 */

 //TODO: Posible mejora, que se pueda elegir desde consola e implementar el modo 1 solo jugador.
fn main() {
    game([('R','S'),('S','R'),('P','S')]);
    game([('P','R'),('S','P'),('R','S')]);
    game([('P','S'),('R','S'),('S','S')]);
}

fn game (plays:[(char,char);3]) {
    let win_player1 = [('R','S'),('S','P'),('P','R')];
    let win_player2 = [('S','R'),('P','S'),('R','P')];
    let win_tie = [('S','S'),('R','R'),('P','P')];
    let mut _p1 = 0;let mut _p2 = 0;let mut _tie = 0;

    for pair in plays {
        if win_player1.contains(&pair){
            _p1 +=1;
        }else if win_player2.contains(&pair){
            _p2 += 1;
        }else if win_tie.contains(&pair){
            _tie += 1;
        }else{
            println!("El par Introducido es incorrecto");
            println!("-------------SALIENDO DEL PROGRAMA-------------");
        }
    }
    println!("\nEntrada: {:?}.  Resultado: {}",plays,check_winner(_p1, _p2));
    
}

fn check_winner(p1:i32,p2:i32) -> &'static str {
    if p1 > p2{
        return "Player 1";
    }else if p2 > p1{
        return "Player 2";
    }else{
        return "Tie";
    }
}
