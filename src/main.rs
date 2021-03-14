use std::env;


fn main() {
    let args: Vec<String> = env::args().collect();
    let x  = 44.0; 
    let nombre = args[1].to_string();

    let name =  String::from(nombre);

    println!("Hola, {:?} son {1:?}", name, x);

    // println!("Hola, {}!", name);
}
