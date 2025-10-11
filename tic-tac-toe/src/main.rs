//! # Tic Tac Toe

use std::io;

enum State {
    Continue,
    Draw,
    End(usize),
}

struct Table {
    data: Vec<char>,
}

impl Table {
    fn is_complete(&self) -> bool {
        for col in self.data.iter() {
            if *col != 'X' && *col != 'O' {
                return false;
            }
        }

        true
    }
    fn check_row(&self, ch: char) -> bool {
        for row in [0, 3, 6] {
            if self.data[row] == ch && self.data[row + 1] == ch && self.data[row + 2] == ch {
                return true;
            }
        }
        false
    }

    fn check_col(&self, ch: char) -> bool {
        for col in [0, 1, 2] {
            if self.data[col] == ch && self.data[col + 3] == ch && self.data[col + 6] == ch {
                return true;
            }
        }
        false
    }

    fn check_diagonals(&self, ch: char) -> bool {
        if self.data[0] == ch && self.data[4] == ch && self.data[8] == ch
            || self.data[2] == ch && self.data[4] == ch && self.data[6] == ch
        {
            return true;
        }
        false
    }

    pub fn print(&self) {
        for i in 0..3 {
            print!(" ");
            for j in 0..3 {
                print!("{}", self.data[i * 3 + j]);
                if j != 2 {
                    print!(" | ")
                };
            }
            if i != 2 {
                println!("\n-----------")
            };
        }

        println!();
    }

    pub fn check(&self) -> State {
        if self.check_row('X') || self.check_col('X') || self.check_diagonals('X') {
            return State::End(1);
        } else if self.check_row('O') || self.check_col('O') || self.check_diagonals('O') {
            return State::End(2);
        } else if self.is_complete() {
            return State::Draw;
        }
        State::Continue
    }
}

fn main() {
    let mut table = Table { data: vec![' '; 9] };
    let mut x_turn = true;
    let mut choice_txt: String = String::new();

    loop {
        choice_txt.clear();
        table.print();
        println!("Enter your choice: ");
        io::stdin().read_line(&mut choice_txt).unwrap();

        match choice_txt.trim().parse::<usize>() {
            Err(e) => {
                println!("Error: {}", e);
                continue;
            }
            Ok(idx) => {
                if idx == 0 {
                    println!("Enter a valid number!");
                    continue;
                }
                if let Some(col) = table.data.get_mut(idx - 1) {
                    if *col == ' ' {
                        *col = if x_turn { 'X' } else { 'O' };
                    } else {
                        println!("This column was already written...");
                        continue;
                    }
                } else {
                    println!("Invalid choice...");
                    continue;
                }

                match table.check() {
                    State::Continue => {
                        x_turn = !x_turn;
                    }
                    State::Draw => {
                        table.print();
                        println!("Draw game!");
                        break;
                    }
                    State::End(winner) => {
                        table.print();
                        println!("Game ended and player {} is the winner!", winner);
                        break;
                    }
                };
            }
        }
    }
}
