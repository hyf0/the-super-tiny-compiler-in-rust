mod test;

#[derive(PartialEq, Eq, Debug)]
pub enum Token {
    ParenLeft,
    ParenRight,
    Name(String),
    Number(String),
}

pub fn tokenizer(input: &str) -> Vec<Token> {
    let input = input.chars().collect::<Vec<char>>();
    let mut tokens = vec![];
    let mut current = 0;
    let input_len = input.len();
    while let Some(&char) = input.get(current) {
        use Token::*;
        match char {
            '(' => tokens.push(ParenLeft),
            ')' => tokens.push(ParenRight),
            ' ' => {}
            '1'..='9' => {
                let mut num_str = String::new();
                while let Some(&num_like @ '0'..='9') = input.get(current) {
                    num_str.push(num_like);
                    current += 1;
                }
                tokens.push(Number(num_str));
                continue;
            }
            'a'..='z' | 'A'..='Z' => {
                let mut name = String::new();
                // or-patterns syntax is experimental
                // while let Some(&char @ 'a'..='z' | &char @ 'A'..='Z') = input.get(current) {
                // }
                while current < input_len {
                    match input[current] {
                        'a'..='z' | 'A'..='Z' => {
                            let char = input[current];
                            name.push(char);
                        }
                        _ => break,
                    }
                    current += 1;
                }
                tokens.push(Name(name));
                continue;
            }
            _ => panic!("I dont know what this character is: {}", char),
        }
        current += 1;
    }
    tokens
}
pub fn parser() {}
pub fn traverser() {}
pub fn transformer() {}
pub fn code_generator() {}
pub fn compiler() {}
