fn main() {
    println!("Conversor!");
    
    let mut celsius = 0;

    // Versão inicial, de 0 a 100, 10 a 10

    loop {
        let fahrenheit = celsius * 9 / 5 + 32;
        println!("{celsius}º C = {fahrenheit} F");
        celsius += 10;

        if celsius > 100 {
            break;
        }
    }

    // Versão com For e Intervalo, de 0 a 100, 1 em 1

    celsius = 0;
    for celsius in 10..=100 {
        let fahrenheit = celsius * 9 / 5 + 32;
        println!("{celsius}º C = {fahrenheit} F");
    }
    
    // Versão com For e Intervalo, de 0 a 100, 10 a 10

    celsius = 0;
    for celsius in (0..=100).step_by(10) {
        let fahrenheit = celsius * 9 / 5 + 32;
        println!("{celsius}º C = {fahrenheit} F");
    }
}

/*
 * println! é um macro, não uma função comum.
 * loop gera um loop sem condição de parada especificada.
*/
