use rand::Rng;
use std::io;
use std::str::FromStr;

fn main() -> io::Result<()> {
    let res: i32 = rand::rng().random_range(1..=100);

    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)?;

        match i32::from_str(&guess.trim()) {
            Err(_) => {
                println!("Insert a valid number!");
            }
            Ok(n) => {
                if n > 100 || n < 0 {
                    println!("Guess outside the valid range..");
                } else if n == res {
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
