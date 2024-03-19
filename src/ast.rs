#[derive(Debug, Clone)]
pub enum AbstractSyntaxTree {
    Const {
        name: String,
        type_annot: String,
        value: Box<AbstractSyntaxTree>,
    },
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
    Fn {
        name: String,
        // args: Vec<String>,
        body: Box<Vec<AbstractSyntaxTree>>,
    },
    Call {
        name: String,
        args: Vec<AbstractSyntaxTree>,
    },
    Block {
        statements: Vec<AbstractSyntaxTree>,
    },
}
