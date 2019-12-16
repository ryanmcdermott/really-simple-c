#[derive(Debug, Clone)]
pub enum NodeType {
    // Number literal
    Num(i32),
}

#[derive(Debug, Clone)]
pub struct Node {
    pub op: NodeType,  
    pub ty: Box<Type>, 
}

#[derive(Debug, Clone)]
pub struct Node {
    pub op: NodeType, 
    pub ty: Box<Type>, 
}

pub fn parse(tokens: &Vec<Token>) -> Vec<Node> {
    let mut ast = vec![];
    ast
}