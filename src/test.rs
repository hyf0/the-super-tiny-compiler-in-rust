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
        use crate::Node::*;
        let right = Program(vec![Box::new(CallExpression {
            name: "add".to_string(),
            params: vec![
                Box::new(NumberLiteral("2".to_string())),
                Box::new(CallExpression {
                    name: "subtract".to_string(),
                    params: vec![
                        Box::new(NumberLiteral("4".to_string())),
                        Box::new(NumberLiteral("2".to_string())),
                    ],
                }),
            ],
        })]);
        assert_eq!(parser(&tokenizer(super::INPUT)), right);
    }
}
