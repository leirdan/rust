/* Variável estática MUTÁVEL. Por padrão, o compilador não permite a manipulação segura pelo fato
 * de haver concorrência entre processos do SO que podem acessar o dado. Para acessá-la, é
 * necessário o uso de um bloco UNSAFE. */
static mut counter: i32 = 0;

fn example() {
    for _ in 0..1000000 {
        register_counter();
    }
}

fn register_counter() {
    unsafe {
        counter += 1;
    }
}

fn get_counter() -> i32 {
    unsafe { counter }
}

/*fn main() {
    for _ in 0..10 {
        register_counter();
    }
    println!("Counter: {}", get_counter());
}*/
fn main() {
    let t1 = std::thread::spawn(example);
    let t2 = std::thread::spawn(example);
    let _ = t1.join();
    let _ = t2.join();
    println!("Counter: {}", get_counter());
}
