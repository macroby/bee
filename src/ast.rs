#[derive(Debug, Clone)]
pub struct Document {
    pub constants: Vec<Constant>,
    pub functions: Vec<Function>,
}
#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub body: Vec<AbstractSyntaxTree>,
}
#[derive(Debug, Clone)]
pub struct Constant {
    pub name: String,
    pub type_annot: String,
    pub value: AbstractSyntaxTree,
}

#[derive(Debug, Clone)]
pub enum AbstractSyntaxTree {
    Let {
        name: String,
        type_annot: String,
        value: Box<AbstractSyntaxTree>,
    },
    Int {
        value: usize,
    },
    String {
        value: String,
    },
    Name {
        name: String,
    },
    UpName {
        name: String,
    },
    Plus {
        left: Box<AbstractSyntaxTree>,
        right: Box<AbstractSyntaxTree>,
    },
    Minus {
        left: Box<AbstractSyntaxTree>,
        right: Box<AbstractSyntaxTree>,
    },
    LtGt {
        left: Box<AbstractSyntaxTree>,
        right: Box<AbstractSyntaxTree>,
    },
    And {
        left: Box<AbstractSyntaxTree>,
        right: Box<AbstractSyntaxTree>,
    },
    Or {
        left: Box<AbstractSyntaxTree>,
        right: Box<AbstractSyntaxTree>,
    },
    Not {
        value: Box<AbstractSyntaxTree>,
    },
    Call {
        name: String,
        args: Vec<AbstractSyntaxTree>,
    },
    Block {
        statements: Vec<AbstractSyntaxTree>,
    },
}
