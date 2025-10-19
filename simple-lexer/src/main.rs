fn next(input: &str) -> Result<(usize, &str, &str), Option<usize>> {
    let mut start_idx = 0;
    for (i, c) in input.char_indices() {
        if c.is_whitespace() || c == '🦀' {
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
    } else if ['+', '-', '*', '/', '🐧'].contains(&first_c) {
        let end = first_i + first_c.len_utf8();
        Ok((start_idx + first_i, &slice[first_i..end], &slice[end..]))
    } else {
        Err(Some(start_idx + first_i))
    }
}

fn main() {
    let mut input_line = String::new();
    std::io::stdin().read_line(&mut input_line).unwrap();
    let mut input: &str = input_line.as_str();
    let mut offset = 1;

    while !input.is_empty() {
        let res = next(&input);
        match res {
            Ok((pos, symbol, substr)) => {
                let char_pos = input[..pos].chars().count() + offset;
                print!("({}, {}) ", symbol, char_pos);

                offset = char_pos + symbol.chars().count();
                input = substr;
            }
            Err(Some(pos)) => {
                let char_pos = input[..pos].chars().count() + offset;
                print!("Erro na posição {} ", char_pos);
                break;
            }
            Err(None) => {
                break;
            }
        }
    }
}
