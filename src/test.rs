#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn tokenizer_test() {
        use crate::Token::*;
        let input = "(add 2 (subtract 4 2))";
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
        assert_eq!(tokenizer(input), tokens);
    }
}
