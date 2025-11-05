use std::collections::VecDeque;

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

#[derive(Clone)]
enum Expression<'a> {
    Number(i64),
    Symbol(char),
    String(Vec<&'a str>),
    Exp {
        left: Box<Expression<'a>>,
        right: Box<Expression<'a>>,
        operation: Box<Expression<'a>>,
    },
}

impl Expression<'_> {
    fn evaluate(&self) -> Option<i64> {
        match self {
            Expression::Number(num) => Some(*num),
            Expression::Symbol(sym) => None,
            Expression::String(_) => None,
            Expression::Exp {
                left,
                right,
                operation,
            } => {
                let l = left.evaluate();
                let r = right.evaluate();

                if let Some(v_l) = l
                    && let Some(v_r) = r
                {
                    if let Expression::Symbol(sym) = **operation {
                        match sym {
                            '+' => Some(v_l + v_r),
                            '-' => Some(v_l - v_r),
                            '*' => Some(v_l * v_r),
                            '/' => {
                                if v_r == 0 {
                                    println!("Div by 0 is not allowed");
                                    None
                                } else {
                                    Some(v_l / v_r)
                                }
                            }
                            '%' => Some(v_l % v_r),
                            _ => None,
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
        }
    }

    fn print(&self) {}

    fn convert_tokens_to_exp() {}

    fn tree(&self) {
        self.tree_rec("", true, true);
    }

    fn tree_rec<'a>(&self, tab: &str, last: bool, root: bool) {
        let create_exp = |v: &mut VecDeque<Expression<'a>>| {
            let mut idx: Option<usize> = None;
            for (i, ex) in v.iter().enumerate() {
                if let Expression::Symbol(sym) = ex {
                    match sym {
                        '*' | '%' | '/' => {
                            idx = Some(i);
                            break;
                        }
                        '+' | '-' => {
                            idx = Some(i);
                        }
                        _ => {}
                    }
                }
            }

            let Some(mut i) = idx else { todo!() };

            let l = Box::new(v.remove(i - 1).unwrap());
            let op = Box::new(v.remove(i - 1).unwrap());
            i = if i > v.len() { v.len() } else { i };
            let r = Box::new(v.remove(i - 1).unwrap());

            Expression::Exp {
                left: l,
                right: r,
                operation: op,
            }
        };

        match self {
            Expression::Number(num) => {
                println!(
                    "{}{}{}",
                    tab,
                    if root {
                        ""
                    } else if last {
                        "└─ "
                    } else {
                        "├─ "
                    },
                    num
                );
            }
            Expression::Symbol(sym) => {
                println!(
                    "{}{}{}",
                    tab,
                    if root {
                        ""
                    } else if last {
                        "└─ "
                    } else {
                        "├─ "
                    },
                    sym
                );
            }
            Expression::String(tokens) => {
                let mut symbols: VecDeque<Expression> = VecDeque::new();
                let mut expressions: Vec<Expression> = Vec::new();

                for &token in tokens {
                    match token {
                        "+" | "-" | "%" | "/" | "*" => {
                            symbols.push_back(Expression::Symbol(token.parse().unwrap()))
                        }
                        ")" => {
                            let exp = create_exp(&mut symbols);
                            symbols.push_front(exp.clone());
                            expressions.push(exp);
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
                    let exp = create_exp(&mut symbols);
                    symbols.push_front(exp.clone());
                    expressions.push(exp);
                }

                if let Some(exp) = expressions.pop() {
                    exp.tree();
                }
            }
            Expression::Exp {
                left,
                right,
                operation,
            } => {
                operation.tree_rec(tab, false, root);
                let pre: &str = &format!("{}{}", tab, if last { "   " } else { "│  " });
                left.tree_rec(&pre, false, false);
                right.tree_rec(&pre, true, false);
            }
        }
    }
}

fn main() {
    let str = "10 + 20 * 30";
    let tokens = retrieve_tokens(str);

    let expr = Expression::String(*tokens);
    expr.tree();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exp1() {
        // let str = "(-(10 + 20) + 30 + 40 + (50 + 60)) * -5"; // TODO: TÁ COM PROBLEMA AINDA
        Expression::String(*retrieve_tokens("10 + 20")).tree();
        Expression::String(*retrieve_tokens("10 / 0")).tree();
        Expression::String(*retrieve_tokens("10 + 20")).tree();
        Expression::String(*retrieve_tokens("(10 + 20) * 30")).evaluate();
        Expression::String(*retrieve_tokens("10 + 20 * 30")).tree();
    }
}
