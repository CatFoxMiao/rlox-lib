use crate::token::{ Token};

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    has_error: bool,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Scanner {
            source: source,
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
            has_error: false,
        }
    }

    pub fn show(&self) -> (&String, &Vec<Token>, &usize, &usize, &usize, &bool) {
        return (
            &self.source,
            &self.tokens,
            &self.start,
            &self.current,
            &self.line,
            &self.has_error,
        );
    }
}

#[cfg(test)]
mod test_scanner {



    use std::vec;

    use super::*;
    #[test]
    fn test_new() {
        let source_str = String::from("hello world");
        let scanner = Scanner::new(source_str);
        let (source ,tokens,start,current,line,has_error) = scanner.show();
        assert_eq!(*source, "hello world".to_string());
        assert_eq!(*tokens, vec![]);
        assert_eq!(*start,0);
        assert_eq!(*current,0);
        assert_eq!(*line,1);
        assert_eq!(*has_error,false);

    }
}
