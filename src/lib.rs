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
            _ => panic!("I don't know what this character is: {}", char),
        }
        current += 1;
    }
    tokens
}

pub mod ast {
    #[derive(PartialEq, Eq, Debug)]
    pub struct Program {
        pub body: Vec<Node>
    }

    #[derive(PartialEq, Eq, Debug)]
    pub struct CallExpression {
        pub name: String,
        pub params: Vec<Node>,
    }
    #[derive(PartialEq, Eq, Debug)]
    pub struct NumberLiteral {
        pub value: String
    }

    #[derive(PartialEq, Eq, Debug)]
    pub enum Node {
        Program(Program),
        CallExpression(CallExpression),
        NumberLiteral(NumberLiteral),
    }

    impl Node {
        pub fn new_program(body: Vec<Node>) -> Node {
            Node::Program(Program{ body })
        }
        pub fn new_call_expression(name: String, params: Vec<Node>) -> Node {
            Node::CallExpression(CallExpression { name, params })
        }
        pub fn new_number_literal(value: String) -> Node {
            Node::NumberLiteral(NumberLiteral{ value })
        }

    }

}


pub fn parser(tokens: &Vec<Token>) -> ast::Node {
    use ast::Node;
    fn walk(tokens: &Vec<Token>, current: &mut usize) -> ast::Node {
        use Token::*;
        let token = &tokens[*current];
        let node = match token {
            Number(value) => Node::new_number_literal(value.clone()),
            ParenLeft => {
                *current += 1;
                let node = if let &Name(ref token_name) = &tokens[*current] {
                    *current += 1;
                    let mut params: Vec<ast::Node> = Vec::new();
                    while *current < tokens.len() {
                        let token = &tokens[*current];
                        if let ParenRight = token {
                            *current += 1;
                            break;
                        } else {
                            params.push(walk(&tokens, current));
                        }
                        *current += 1;
                    }
                    Node::new_call_expression(token_name.clone(), params)
                } else {
                    panic!("`(` must follow with a Name");
                };
                node
            }
            unexpected_token => panic!("Type Error {:?}", unexpected_token),
        };
        node
    }
    let mut body: Vec<ast::Node> = Vec::new();
    let mut current: usize = 0;

    while current < tokens.len() {
        body.push(walk(&tokens, &mut current));
    }
    let program = Node::new_program(body);
    program
}


pub trait Visitor {
    fn enter_program(_n: &mut ast::Program) {

    }
    fn leave_program(_n: &mut ast::Program) {

    }
    fn enter_call_expression(_n: &mut ast::CallExpression) {

    }
    fn leave_call_expression(_n: &mut ast::CallExpression) {

    }
    fn enter_number_literal(_n: &mut ast::NumberLiteral) {

    }
    fn leave_number_literal(_n: &mut ast::NumberLiteral) {

    }
}

pub fn traverser(ast: &ast::Node, visitor: impl Visitor) -> !{
    panic!("to be impl")
}

pub fn transformer() {}
pub fn code_generator() {}
pub fn compiler() {}
