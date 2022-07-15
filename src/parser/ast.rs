use im::Vector;

#[derive(PartialEq, Debug, Clone)]
pub enum ASTNode {
    List(Vector<ASTNode>),
    NumberLiteral(i64),
    StringLiteral(String),
    Identifier(String),
    Quote(Box<ASTNode>),
    BoolLiteral(bool),
    Null,
    _RParen,
}

impl ASTNode {
    pub fn as_list(&self) -> Option<Vector<ASTNode>> {
        match self {
            ASTNode::List(x) => Some(x.clone()),
            _ => None
        }
    }

    pub fn as_number(&self) -> Option<i64> {
        match self {
            ASTNode::NumberLiteral(i) => Some(*i),
            _ => None
        }
    }

    pub fn as_string(&self) -> Option<String> {
        match self {
            ASTNode::StringLiteral(x) => Some(x.clone()),
            _ => None
        }
    }

    pub fn as_id(&self) -> Option<String> {
        match self {
            ASTNode::Identifier(x) => Some(x.clone()),
            _ => None
        }
    }

    pub fn as_quote(&self) -> Option<Box<ASTNode>> {
        match self {
            ASTNode::Quote(x) => Some(x.clone()),
            _ => None
        }
    }
}