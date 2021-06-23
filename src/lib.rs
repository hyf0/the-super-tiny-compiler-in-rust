mod test;

#[derive(PartialEq, Eq, Debug)]
pub enum Token {
    ParenLeft,
    ParenRight,
    Name(String),
    Number(String),
    Str(String),
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
            '"' => {
                let mut str = String::new();
                current += 1;
                while let Some(&c) = input.get(current) {
                    if c == '"' {
                        current += 1;
                        break;
                    } else {
                        str.push(c);
                        current += 1;
                    }
                }
                tokens.push(Str(str));
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
        pub body: Vec<Node>,
    }

    #[derive(PartialEq, Eq, Debug)]
    pub struct CallExpression {
        pub name: String,
        pub params: Vec<Node>,
    }
    #[derive(PartialEq, Eq, Debug)]
    pub struct NumberLiteral {
        pub value: String,
    }
    #[derive(PartialEq, Eq, Debug)]
    pub struct StringLiteral {
        pub value: String,
    }

    #[derive(PartialEq, Eq, Debug)]
    pub enum Node {
        Program(Program),
        CallExpression(CallExpression),
        NumberLiteral(NumberLiteral),
        StringLiteral(StringLiteral),
    }

    impl Node {
        pub fn new_program(body: Vec<Node>) -> Node {
            Node::Program(Program { body })
        }
        pub fn new_call_expression(name: String, params: Vec<Node>) -> Node {
            Node::CallExpression(CallExpression { name, params })
        }
        pub fn new_number_literal(value: String) -> Node {
            Node::NumberLiteral(NumberLiteral { value })
        }
        pub fn new_string_literal(value: String) -> Node {
            Node::StringLiteral(StringLiteral { value })
        }
    }
    pub mod visit {
        pub trait Visitor {
            fn enter_program(&mut self, _node: &super::Program) {}
            fn exit_program(&mut self, _node: &super::Program) {}
            fn enter_call_expression(&mut self, _node: &super::CallExpression, _parent: &super::Node) {}
            fn exit_call_expression(&mut self, _node: &super::CallExpression, _parent: &super::Node) {}
            fn enter_number_literal(&mut self, _node: &super::NumberLiteral, _parent: &super::Node) {}
            fn exit_number_literal(&mut self, _node: &super::NumberLiteral, _parent: &super::Node) {}
            fn enter_string_literal(&mut self, _node: &super::StringLiteral, _parent: &super::Node) {}
            fn exit_string_literal(&mut self, _node: &super::StringLiteral, _parent: &super::Node) {}
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
            Str(value) => Node::new_string_literal(value.clone()),
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

pub fn traverser(ast: &ast::Node, visitor: &mut impl ast::visit::Visitor) {
    use ast::Node;
    fn traverser_node(visitor: &mut impl ast::visit::Visitor, node_to_be_match: &Node, parent: Option<&Node>) {
        match node_to_be_match {
            n @ Node::Program(_) => {
                if let Node::Program(p) = n {
                    visitor.enter_program(p);
                    let body = &p.body;
                    traverser_vec(visitor, body, Some(n));
                    visitor.exit_program(p);
                }
            }
            n @ Node::CallExpression(_) => {
                if let Node::CallExpression(c) = n {
                    visitor.enter_call_expression(c, parent.unwrap());
                    traverser_vec(visitor, &c.params, Some(n));
                    visitor.exit_call_expression(c, parent.unwrap());
                }
            }
            n @ Node::NumberLiteral(_) => {
                if let Node::NumberLiteral(v) = n {
                    visitor.enter_number_literal(v, parent.unwrap());
                    visitor.exit_number_literal(v, parent.unwrap());
                }
            }
            n @ Node::StringLiteral(_) => {
                if let Node::StringLiteral(v) = n {
                    visitor.enter_string_literal(v, parent.unwrap());
                    visitor.exit_string_literal(v, parent.unwrap());
                }
            }
        }
    }

    fn traverser_vec(visitor: &mut impl ast::visit::Visitor, vec: &Vec<Node>, parent: Option<&ast::Node>) {
        vec.iter()
            .for_each(|node| traverser_node(visitor, node, parent))
    }
    traverser_node(visitor, ast, None);
}

pub mod ast_new {
    #[derive(PartialEq, Eq, Debug)]
    pub struct Program {
        pub body: Vec<Node>,
    }
    #[derive(PartialEq, Eq, Debug)]
    pub struct Identifier {
        pub name: String,
    }

    #[derive(PartialEq, Eq, Debug)]
    pub struct CallExpression {
        pub callee: Identifier,
        pub arguments: Vec<Node>,
    }
    #[derive(PartialEq, Eq, Debug)]
    pub struct ExpressionStatement {
        pub expression: CallExpression,
    }
    #[derive(PartialEq, Eq, Debug)]
    pub struct NumberLiteral {
        pub value: String,
    }

    #[derive(PartialEq, Eq, Debug)]
    pub struct StringLiteral {
        pub value: String,
    }

    #[derive(PartialEq, Eq, Debug)]
    pub enum Node {
        Program(Program),
        Identifier(Identifier),
        CallExpression(CallExpression),
        ExpressionStatement(ExpressionStatement),
        NumberLiteral(NumberLiteral),
        StringLiteral(StringLiteral),
    }


    impl Node {
        pub fn new_program(body: Vec<Node>) -> Node {
            Node::Program(Program { body })
        }
        pub fn new_identifier(name: String) -> Node {
            Node::Identifier(Identifier { name })
        }
        pub fn new_call_expression(callee: Identifier, arguments: Vec<Node>) -> Node {
            Node::CallExpression(CallExpression {
                callee,
                arguments,
            })
        }
        pub fn new_expression_statement(expression: CallExpression) -> Node {
            Node::ExpressionStatement(ExpressionStatement { expression })
        }
        pub fn new_number_literal(value: String) -> Node {
            Node::NumberLiteral(NumberLiteral { value })
        }
        pub fn new_string_literal(value: String) -> Node {
            Node::StringLiteral(StringLiteral { value })
        }

        pub fn unwrap_program(self) -> Program {
            if let  Node::Program(p) = self {
                p
            } else {
                panic!("ops!");
            }
        }
    }
}

pub fn transformer(ast: &mut ast::Node) -> ast_new::Program {
    #[derive(PartialEq, Eq, Debug)]
    struct State {
        pub stack: Vec<ast_new::Node>,
    }
    impl ast::visit::Visitor for State {
        fn enter_number_literal(&mut self, node: &ast::NumberLiteral, _parent: &ast::Node) {
            let new_node = ast_new::Node::new_number_literal(node.value.clone());
            self.stack.push(new_node);
        }
        fn enter_string_literal(&mut self, node: &ast::StringLiteral, _parent: &ast::Node) {
            let new_node = ast_new::Node::new_string_literal(node.value.clone());
            self.stack.push(new_node);
        }
        fn enter_call_expression(&mut self, node: &ast::CallExpression, parent: &ast::Node) {
            println!("enter_call_expression _parent: {:?}", parent);
            let id = ast_new::Identifier { name: node.name.clone() };
            let exp = ast_new::CallExpression{ callee: id, arguments: vec![], };
            if let ast::Node::CallExpression(_) = parent {
                self.stack.push(ast_new::Node::CallExpression(exp));
            } else {
                let exp_s = ast_new::Node::new_expression_statement(exp);
                self.stack.push(exp_s);
            }
        }
        fn enter_program(&mut self, _node: &ast::Program) {
            let p = ast_new::Node::new_program(vec![]);
            self.stack.push(p);
        }
    }
    // let mut p = ast_new::Node::new_program(vec![]);
    let mut s = State { stack: vec![] };
    // s.stack.push(p);
    traverser(ast, &mut s);
    println!("\n\ns.stack: {:?}\n", s.stack);
    let mut collected = vec![];
    while let Some(n) = s.stack.pop() {
        use ast_new::Node;
        match n {
            bind @ Node::NumberLiteral(_) => {
                collected.push(bind);
            }
            bind @ Node::StringLiteral(_) => {
                collected.push(bind);
            }
            Node::ExpressionStatement(mut es) => {
                collected.reverse(); // 由于借助了栈来还原参数，导致参数被逆序了，暂时没有好方法，只能强拧回来
                es.expression.arguments = collected;
                collected = vec![Node::ExpressionStatement(es)];
            }
            Node::CallExpression(mut c) => {
                collected.reverse();
                c.arguments = collected;
                collected = vec![Node::CallExpression(c)];
            }
            Node::Program(mut p) => {
                collected.reverse();
                p.body = collected;
                collected = vec![Node::Program(p)];
            }
            _node => panic!("unexpected {:?}", _node)

        }
    }
    println!("collected: {:?}\n", collected);
    collected.pop().unwrap().unwrap_program()
}
pub fn code_generator() {}
pub fn compiler() {}
