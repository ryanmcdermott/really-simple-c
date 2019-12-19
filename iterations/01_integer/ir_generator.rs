#[derive(Debug, Clone, PartialEq)]
pub enum IROp {
    Mov,
    Return,
}

#[derive(Clone, Debug)]
pub struct Function {
    pub name: String,
    pub ir: Vec<IR>,
    pub stacksize: usize,
}

#[derive(Clone, Debug)]
pub enum IRType {
    Reg,
}

#[derive(Debug, Clone)]
pub struct IR {
    pub op: IROp,
    pub lhs: Option<usize>,
    pub rhs: Option<usize>,
}

pub fn gen_ir(nodes: Vec<Node>) -> Vec<Function> {
    let mut ir_nodes = vec![];
    ir_nodes;
}