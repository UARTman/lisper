use im::Vector;
use logos::{Lexer, Logos};

pub mod ast;
pub use ast::ASTNode;

#[derive(Logos, Debug, PartialEq)]
enum Token {
    #[regex("\"[^\"]*\"", |lex| lex.slice().to_string())]
    StringLiteral(String),

    #[regex("-?[0-9]+", |lex| lex.slice().parse(), priority = 2)]
    NumberLiteral(i64),

    #[token("(")]
    Open,

    #[token(")")]
    Close,

    #[token("'")]
    Quote,

    #[token("true")]
    True,

    #[token("false")]
    False,

    #[token("null")]
    Null,

    #[regex("[a-zA-Z_+-<>*/|?=]+", |lex| lex.slice().to_string())]
    Identifier(String),

    #[error]
    // We can also use this variant to define whitespace,
    // or any other matches we wish to skip.
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Error,
}



pub struct Parser<'source> {
    lex: Lexer<'source, Token>,
}

impl<'source> Parser<'source> {
    pub fn new(s: &'source str) -> Self {
        Parser {
            lex: Token::lexer(s),
        }
    }
}

impl<'source> Iterator for Parser<'source> {
    type Item = ASTNode;

    fn next(&mut self) -> Option<Self::Item> {
        match self.lex.next()? {
            Token::Open => {
                let mut tokens = Vector::new();
                loop {
                    let expr = self.next()?;
                    if expr == ASTNode::_RParen {
                        break;
                    }
                    tokens.push_back(expr);
                }
                Some(ASTNode::List(tokens))
            }
            Token::Close => Some(ASTNode::_RParen),
            Token::Quote => {
                let expr = self.next()?;
                Some(ASTNode::Quote(Box::new(expr)))
            }
            Token::StringLiteral(mut s) => {
                s.pop();
                s.remove(0);
                Some(ASTNode::StringLiteral(s))
            }
            Token::NumberLiteral(i) => Some(ASTNode::NumberLiteral(i)),
            Token::Identifier(s) => Some(ASTNode::Identifier(s)),
            Token::Error => None,
            Token::True => Some(ASTNode::BoolLiteral(true)),
            Token::False => Some(ASTNode::BoolLiteral(false)),
            Token::Null => Some(ASTNode::Null),
        }
    }
}

#[cfg(test)]
mod test {
    use im::vector;

    use crate::parser::*;

    fn parse_string(s: &str) -> Vec<ASTNode> {
        Parser::new(s).collect()
    }

    fn quote(n: ASTNode) -> ASTNode {
        ASTNode::Quote(Box::new(n))
    }

    #[test]
    fn test_list() {
        let testing = "(call \"1\" 2 test)";
        let nodes = parse_string(testing);
        for i in nodes {
            println!("{i:?}");
        }
    }

    #[test]
    fn test_string() {
        let input = "\"12345`'sxz   asc~''sdf\"";
        let nodes = parse_string(input);
        assert_eq!(
            nodes[0],
            ASTNode::StringLiteral("12345`'sxz   asc~''sdf".into())
        );
    }

    #[test]
    fn test_numbers() {
        let input = "-100500 100500";
        let nodes = parse_string(input);
        assert_eq!(nodes[0], ASTNode::NumberLiteral(-100500));
        assert_eq!(nodes[1], ASTNode::NumberLiteral(100500));
    }

    #[test]
    fn test_quotes() {
        let input = "'123 'abc '\"abc\" '(a b c d)";
        let nodes = parse_string(input);
        assert_eq!(nodes[0], quote(ASTNode::NumberLiteral(123)));
        assert_eq!(nodes[1], quote(ASTNode::Identifier("abc".into())));
        assert_eq!(nodes[2], quote(ASTNode::StringLiteral("abc".into())));
        assert_eq!(
            nodes[3],
            quote(ASTNode::List(vector![
                ASTNode::Identifier("a".into()),
                ASTNode::Identifier("b".into()),
                ASTNode::Identifier("c".into()),
                ASTNode::Identifier("d".into()),
            ]))
        );
    }
}
