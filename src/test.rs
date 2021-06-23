static INPUT: &str = "(add 2 (subtract 4 2))";

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn tokenizer_test() {
        use crate::Token::*;
        let tokens = vec![
            ParenLeft,
            Name("add".to_string()),
            Number("2".to_string()),
            ParenLeft,
            Name("subtract".to_string()),
            Number("4".to_string()),
            Number("2".to_string()),
            ParenRight,
            ParenRight,
        ];
        assert_eq!(tokenizer(super::INPUT), tokens);
    }
    #[test]
    fn parser_test() {
        let right = {
            use crate::ast::Node;
            Node::new_program(vec![Node::new_call_expression(
                "add".to_owned(),
                vec![
                    Node::new_number_literal("2".to_string()),
                    Node::new_call_expression(
                        "subtract".to_owned(),
                        vec![
                            Node::new_number_literal("4".to_owned()),
                            Node::new_number_literal("2".to_owned()),
                        ],
                    ),
                ],
            )])
        };
        assert_eq!(parser(&tokenizer(super::INPUT)), right);
    }
    #[test]
    fn transformer_test() {
        use crate::ast_new::{Node};
        let right = ast_new::Program { body: vec![
            Node::new_expression_statement(ast_new::CallExpression {
                callee: ast_new::Identifier { name: "add".to_string() },
                arguments: vec![
                    Node::new_number_literal("2".to_owned()),
                    Node::new_call_expression(
                        ast_new::Identifier { name: "subtract".to_owned() },
                        vec![
                            Node::new_number_literal("4".to_owned()),
                            Node::new_number_literal("2".to_owned()),
                        ],
                    ),
                ],
            })
        ] };
        println!("{:?}", transformer(&mut parser(&tokenizer(super::INPUT))));
        assert_eq!(transformer(&mut parser(&tokenizer(super::INPUT))), right);
        // transformer( tokens);
    }
}
