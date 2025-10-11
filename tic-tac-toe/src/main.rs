//! # Tic Tac Toe

use std::io;

enum State {
    Continue,
    Draw,
    End(char),
}

struct Table {
    data: Vec<char>,
}

impl Table {
    fn is_complete(&self) -> bool {
        for col in self.data.iter() {
            if *col != 'X' || *col != 'O' {
                return false;
            }
        }

        true
    }
}

fn check_row(table: &Table, ch: char) -> bool {
    for row in [0, 3, 6] {
        if table.data[row] == ch && table.data[row + 1] == ch && table.data[row + 2] == ch {
            return true;
        }
    }
    false
}

fn check_col(table: &Table, ch: char) -> bool {
    for col in [0, 1, 2] {
        if table.data[col] == ch && table.data[col + 3] == ch && table.data[col + 6] == ch {
            return true;
        }
    }
    false
}

fn check_diagonals(table: &Table, ch: char) -> bool {
    if table.data[0] == ch && table.data[4] == ch && table.data[8] == ch
        || table.data[2] == ch && table.data[4] == ch && table.data[6] == ch
    {
        return true;
    }
    false
}

fn check_table(table: &Table) -> State {
    if check_row(table, 'X') || check_col(table, 'X') || check_diagonals(table, 'X') {
        return State::End('X');
    } else if check_row(table, 'O') || check_col(table, 'O') || check_diagonals(table, 'O') {
        return State::End('O');
    } else if table.is_complete() {
        return State::Draw;
    }
    State::Continue
}
fn print_table(table: &Table) {
    for i in 0..3 {
        print!(" ");
        for j in 0..3 {
            print!("{}", table.data[i * 3 + j]);
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

fn main() {
    let mut table = Table { data: vec![' '; 9] };
    let mut x_turn = true;
    let mut choice_txt: String = String::new();

    loop {
        choice_txt.clear();
        print_table(&table);
        println!("Enter your choice: ");
        io::stdin().read_line(&mut choice_txt).unwrap();

        match choice_txt.trim().parse::<usize>() {
            Err(e) => {
                println!("Enter a valid number!");
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

                match check_table(&table) {
                    State::Continue => {
                        x_turn = !x_turn;
                    }
                    State::Draw => {
                        print_table(&table);
                        println!("Draw game!");
                        break;
                    }
                    State::End(winner) => {
                        print_table(&table);
                        println!(
                            "Game ended and player {} is the winner!",
                            if winner == 'X' { &"1" } else { &"2" }
                        );
                        break;
                    }
                };
            }
        }
    }
}
