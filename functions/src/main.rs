// Funções compostas
fn map(t: (i32, i32), f: fn(i32) -> i32, g: fn(i32, i32) -> i32) -> i32 {
    g(f(t.0), f(t.1))
}

// Uso de lambda functions como argumentos!
fn main() {
    println!(" functions: {:?}", map((10, 20), |x| x + 1, |x, y| x * y));
}
