fn next(input: &str) -> Result<(usize, &str, &str), Option<usize>> {
    if input.is_empty() {
        return Err(None);
    }

    match input.char_indices().nth(0) {
        None => Err(None),
        Some((idx, ch)) => {
            if ch.is_whitespace() {
                Ok((idx + 1, "", &input[idx + 1..]))
            } else if ch == '🦀' {
                Ok((idx + 1, "", &input[idx + 4..]))
            } else if ch == '+' || ch == '-' || ch == '/' || ch == '*' {
                Ok((idx + 1, &input[idx..=idx], &input[idx + 1..]))
            } else if ch == '🐧' {
                Ok((idx + 1, &input[idx..idx + 4], &input[idx + 4..]))
            } else if ch.is_ascii_digit() {
                let mut final_idx = idx;

                for ch2 in input.chars() {
                    if ch2.is_ascii_digit() {
                        final_idx += 1;
                    } else {
                        break;
                    }
                }

                Ok((final_idx, &input[idx..final_idx], &input[final_idx..]))
            } else {
                Err(Some(idx))
            }
        }
    }
}

fn main() {
    let mut input_line = String::new();
    std::io::stdin().read_line(&mut input_line).unwrap();
    let mut input: &str = input_line.trim();
    let mut offset = 1;

    while !input.is_empty() {
        match next(input) {
            Ok((idx, element, str)) => {
                if !element.is_empty() {
                    print!("(\"{}\", {}) ", element, offset);
                    offset += idx;
                } else {
                    offset += 1;
                }
                input = str;
            }
            Err(Some(idx)) => {
                offset += idx;
                print!("Erro na posição {}", offset);
                break;
            }
            Err(None) => offset += 1,
        }
    }
}
