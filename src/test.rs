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
            Node::new_program(vec![
                Node::new_call_expression("add".to_owned(), vec![
                    Node::new_number_literal("2".to_string()),
                    Node::new_call_expression("subtract".to_owned(), vec![
                        Node::new_number_literal("4".to_owned()),
                        Node::new_number_literal("2".to_owned()),
                    ])
                ]),
            ])
        };
        assert_eq!(parser(&tokenizer(super::INPUT)), right);
    }
}
