use rand::Rng;
use std::io;
use std::str::FromStr;

fn main() -> io::Result<()> {
    let rng = rand::rng();
    let res: i32 = rng.random_range(1..=100);

    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)?;

        match i32::from_str(&guess.trim()) {
            Err(e : Err) => {
                println!("Insert a valid number!");
            }
            Ok(n: i32) => {
                if n == res {
                    println!("You won!");
                    break;
                } else if n < res {
                    println!("Guess is too low..");
                } else if n > res {
                    println!("Guess is too high!");
                }
            }
        }
    }

    Ok(())
}
