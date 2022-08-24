/* 
! retosdeprogramacion.com reto semanal #11:
 * Escribe una función que reciba un texto y retorne verdadero o
 * falso (Boolean) según sean o no palíndromos.
 * Un Palíndromo es una palabra o expresión que es igual si se lee
 * de izquierda a derecha que de derecha a izquierda.
 * NO se tienen en cuenta los espacios, signos de puntuación y tildes.
 * Ejemplo: Ana lleva al oso la avellana.
 */

 use std::{collections::HashMap};

fn main() {
    if is_palindromo(String::from("Aná lleva; al oso, la avellana?")){
        println!("Es un Palindromo");
    }else{
        println!("NO es un Palindromo");
    }
}

fn is_palindromo(s:String) -> bool {
    let mut s2:String = s.chars().rev().collect();
    let mut s1 = s;
    let mut it_is = false;

    s2 = normalice(s2);
    s1 = normalice(s1);

    if s2 == s1{
        it_is = true;
    }
    it_is
    
}
fn normalice (s:String) -> String {
    
    let mut normalize = String::from("");
    let punctuation_symbols = [',',' ', ';', ':', '.', '·', 
    '¿', '?', '¡', '!', '(', ')', '-', '_', '<', '>', '*', '{', '}',
    '[', ']', '^', '%', '/', '|', '\\'];

    let acents = HashMap::from([('á','a'),('à', 'a'),
    ('ä','a'),('â','a'),('é','e'),('è','e'),('ë', 'e'),( 'ê','e'),
    ('í','i'),('ì','i'),('ï','i'),('î','i'),('ó','o'),('ò','o'),
    ('ö','o'),('ô','o'),('ú','u'),('ù','u'),('ü','u'),('û','u')
    ]);

    for char in s.to_lowercase().chars() {
        if punctuation_symbols.contains(&char){
            continue;
        }
        else if acents.contains_key(&char){
            normalize.push(acents[&char]);
        }
        else{
            normalize.push(char);
        }
    }
    normalize
}
