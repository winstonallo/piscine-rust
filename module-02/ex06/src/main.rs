use ftkit::ARGS;

enum Token<'a> {
    Word(&'a str),
    RedirectStdout,
    RedirectStdin,
    Pipe,
}

fn next_token<'a>(s: &mut &'a str) -> Option<Token<'a>> {
    *s = s.trim_start();
    if s.is_empty() {
        return None;
    }

    let mut char_indices = s.char_indices();
    if let Some((_, c)) = char_indices.next() {
        match c {
            '<' => {
                *s = &s[1..];
                return Some(Token::RedirectStdin);
            }
            '>' => {
                *s = &s[1..];
                return Some(Token::RedirectStdout);
            }
            '|' => {
                *s = &s[1..];
                return Some(Token::Pipe);
            }
            _ => {
                let token_len = s.find(|c: char| c.is_whitespace() || c == '<' || c == '>' || c == '|')
                    .unwrap_or_else(|| s.len());
                let token = &s[..token_len];
                *s = &s[token_len..];
                return Some(Token::Word(token));
            }
        }
    }
    None
}

fn main() {
    let input = &ARGS[1];
    let mut input_str = input;
    while let Some(token) = next_token(&mut input_str) {
        match token {
            Token::Word(word) => println!("Word(\"{}\")", word),
            Token::RedirectStdout => println!("Redirect Stdout"),
            Token::RedirectStdin => println!("Redirect Stdin"),
            Token::Pipe => println!("Pipe"),
        }
    }
}
