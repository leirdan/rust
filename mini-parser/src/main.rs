use std::collections::VecDeque;
use std::num::ParseIntError;

fn next(input: &str) -> Result<(usize, &str, &str), Option<usize>> {
    let mut start_idx = 0;
    for (i, c) in input.char_indices() {
        if c.is_whitespace() {
            start_idx = i + c.len_utf8();
        } else {
            break;
        }
    }

    let slice = &input[start_idx..];
    let mut chars = slice.char_indices();

    let (first_i, first_c) = match chars.next() {
        None => return Err(None),
        Some(c) => c,
    };

    if first_c.is_ascii_digit() {
        let end = slice
            .char_indices()
            .skip_while(|&(_, c)| c.is_ascii_digit())
            .next()
            .map(|(i, _)| i)
            .unwrap_or(slice.len());

        Ok((start_idx + first_i, &slice[first_i..end], &slice[end..]))
    } else if ['+', '-', '*', '/', '%', '(', ')'].contains(&first_c) {
        let end = first_i + first_c.len_utf8();
        Ok((start_idx + first_i, &slice[first_i..end], &slice[end..]))
    } else {
        Err(Some(start_idx + first_i))
    }
}

fn retrieve_tokens(mut input: &str) -> Box<Vec<&str>> {
    let mut tokens = Box::new(Vec::new());
    while !input.is_empty() {
        let res = next(&input);
        match res {
            Ok((_, s, st)) => {
                tokens.push(s);
                input = st;
            }
            Err(None) => {}
            Err(_) => panic!("Invalid expression was found!!"),
        }
    }

    tokens
}

enum Expression<'a> {
    Number(i64),
    Symbol(char),
    String(Vec<&'a str>),
    Exp {
        left: Box<Expression<'a>>,
        right: Box<Expression<'a>>,
        operation: char,
    },
}

impl Expression<'_> {
    fn evaluate(&self) -> Option<i64> {
        None
    }

    fn print(&self) {}

    fn tree(&self) {
        match self {
            Expression::Exp {
                left,
                right,
                operation,
            } => {
                println!("{}", operation);
                left.tree_rec("", false);
                right.tree_rec("", true);
            }
            _ => self.tree_rec("", true),
        }
    }

    fn tree_rec(&self, tab: &str, last: bool) {
        let prefix: &str = if last { "└" } else { "├" };

        match self {
            Expression::Number(num) => {
                println!("{}{} {}", tab, prefix, num);
            }
            Expression::Symbol(sym) => {
                println!("{}{} {}", tab, prefix, sym);
            }
            Expression::String(tokens) => {
                let mut symbols: VecDeque<Expression> = VecDeque::new();
                let mut operations: Vec<char> = Vec::new();

                for &token in tokens {
                    match token {
                        "+" | "-" | "%" | "/" | "*" => operations.push(token.parse().unwrap()),
                        "(" => {}
                        ")" => {
                            let l = Box::new(symbols.pop_front().unwrap());
                            let r = Box::new(symbols.pop_front().unwrap());

                            symbols.push_back(Expression::Exp {
                                left: l,
                                right: r,
                                operation: operations.pop().unwrap(),
                            })
                        }
                        num => match num.parse::<i64>() {
                            Ok(n) => {
                                symbols.push_back(Expression::Number(n));
                            }
                            Err(_) => {}
                        },
                    }
                }

                while symbols.len() != 1 {
                    let l = Box::new(symbols.pop_front().unwrap());
                    let r = Box::new(symbols.pop_front().unwrap());

                    symbols.push_back(Expression::Exp {
                        left: l,
                        right: r,
                        operation: operations.pop().unwrap(),
                    })
                }

                if let Some(exp) = symbols.pop_front() {
                    exp.tree();
                }
            }
            Expression::Exp {
                left,
                right,
                operation,
            } => {
                println!("{}{} {}", tab, prefix, operation);
                let pre = format!("{}{}", tab, if last { " " } else { "| " });
                left.tree_rec(&pre, false);
                right.tree_rec(&pre, true);
            }
        }
    }
}

fn main() {
    let mut tokens: Vec<&str> = Vec::new();
    let str = "(10 + 20) * 30";
    let tokens = retrieve_tokens(str);

    let expr = Expression::String(*tokens);
    expr.tree();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exp1() {
        let str = "10 + 20 * 30"; // TODO: verificar esse erro
        let tokens = retrieve_tokens(str);
        Expression::String(*tokens).tree();

    }
}
