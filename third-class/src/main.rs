use std::io;
use std::str::FromStr;

fn main() -> io::Result<()> {
    /** Leitura de string **/
    let mut name = String::new();
    println!("Insert your name: ");
    io::stdin().read_line(&mut name)?;

    println!("Hello, {}!", name.trim());

    /** Conversão string -> int **/
    let str = "911";
    let x1: i32 = str.parse().unwrap(); // TODO: remover esses unwrap
    let x2 = str.parse::<i32>().unwrap();
    let x3 = i32::from_str(str).unwrap();

    println!("{x1} {x2} {x3}");

    Ok(())
}

/*
 * Tipo de main é um Result com Ok ou Err
 * LEITURA DE STRING
 * Ao ler com read_line, o ? indica que se houver um erro, lança automaticamente
 * O parâmetro para read_line é passado como & e mutável para permitir a ela modificar (atribuir)
 * valor à variável
 * A última linha não precisa de ;, pois é tida como um retorno direto
 *
 * CONVERSÃO STRING -> INT
 * Assinatura de parse : fn parse<T: _> (&self) -> Result<T>. Por self estar na impl de String, o tipo é uma string
 * Em x1, o compilador quem infere que o tipo é i32.
 *
*/
