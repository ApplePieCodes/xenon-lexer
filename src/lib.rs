pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub mod lexer;

#[cfg(test)]
mod tests {
    use crate::lexer::tokenize;

    #[test]
    fn xenon_code() {
        let tokens = tokenize(
            "void main() {
                        return 0;
                    }",
        );

        match tokens {
            Ok(t) => assert!(t.len() == 9),
            Err(e) => panic!("{}", e.to_string()),
        }
    }

    #[test]
    fn unknown_symbol() {
        let tokens = tokenize("^");

        assert!(tokens.is_err());
    }
}
