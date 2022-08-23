/*
 ! retosdeprogramacion.com reto semanal #11:
 * Crea una función que reciba dos cadenas como parámetro (str1, str2)
 * e imprima otras dos cadenas como salida (out1, out2).
 * - out1 contendrá todos los caracteres presentes en la str1 pero NO
 *   estén presentes en str2.
 * - out2 contendrá todos los caracteres presentes en la str2 pero NO
 *   estén presentes en str1.
 */


 fn main(){
    strings_comparator(String::from("GoNzAlO"), String::from("PiLoto"));

 }

 fn strings_comparator(s1:String,s2:String){
    let out1;
    let out2;

    out1 = non_common(s1.clone(), s2.clone());
    out2 = non_common(s2, s1);

    println!("{}",out1);
    println!("{}",out2)
 }

 fn non_common(string1:String,string2:String) -> String {
    let mut out1 = String::from("");
    let mut presence = false;
    for char in string1.to_lowercase().chars(){
        for c in string2.to_lowercase().chars(){
            if char == c {
                presence = true;
            }
        }
        if presence ==false {
            out1.push(char);
        }
        presence = false
    }
    out1
    
}