struct Analyzer<'a> {
    pos: usize,
    next: &'a str,
}

impl<'a> Analyzer<'a> {
    pub fn new(input: &'a str) -> Self {
        Analyzer {
            pos: 1,
            next: input,
        }
    }

    pub fn next(&mut self) -> Result<(usize, &'a str), Option<usize>> {
        if self.next.is_empty() {
            return Err(None);
        }

        match self.next.char_indices().nth(0) {
            None => Err(None),
            Some((idx, ch)) => {
                if ch.is_whitespace() {
                    self.next = &self.next[idx + 1..];
                    let tmp_idx = self.pos;
                    self.pos += 1;
                    Ok((tmp_idx, ""))
                } else if ch.eq(&'🦀') {
                    self.next = &self.next[idx + 4..];
                    let tmp_idx = self.pos;
                    self.pos += 1;
                    Ok((tmp_idx, ""))
                } else if ch == '+' || ch == '-' || ch == '/' || ch == '*' {
                    let tmp = &self.next[idx..=idx];
                    let tmp_idx = self.pos;
                    self.next = &self.next[idx + 1..];
                    self.pos += 1;

                    Ok((tmp_idx, tmp))
                } else if ch == '🐧' {
                    let tmp = &self.next[idx..idx + 4];
                    let tmp_idx = self.pos;
                    self.next = &self.next[idx + 4..];
                    self.pos += 1;

                    Ok((tmp_idx, tmp))

                } else if ch.is_ascii_digit() {
                    let mut final_idx = idx;

                    for ch2 in self.next.chars() {
                        if ch2.is_ascii_digit() {
                            final_idx += 1;
                        } else {
                            break;
                        }
                    }

                    let tmp = &self.next[idx..final_idx];
                    let tmp_idx = self.pos;
                    self.next = &self.next[final_idx..];
                    self.pos += tmp.len();

                    Ok((tmp_idx, tmp))
                } else {
                    Err(Some(idx))
                }
            }
        }
    }
}

fn main() {
    let mut input_line = String::new();
    std::io::stdin().read_line(&mut input_line).unwrap();
    let input: &str = input_line.as_str();

    let mut analyzer = Analyzer::new(input);
    loop {
        match analyzer.next() {
            Ok((pos, symbol)) => {
                if !symbol.is_empty() {
                    print!("(\"{}\", {}) ", symbol, pos);
                }
            }
            Err(Some(idx)) => {
                print!("Erro na posição {}", idx);
                break;
            }
            Err(None) => {
                break;
            }
        }
    }
}
