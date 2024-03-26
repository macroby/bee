#[derive(Debug, Clone)]
pub enum OpCode {
    And {
        arg1: usize,
        arg2: usize,
        arg3: usize,
    },
    Or {
        arg1: usize,
        arg2: usize,
        arg3: usize,
    },
    Not {
        value: usize,
    },
    Add {
        arg1: usize,
        arg2: usize,
        arg3: usize,
    },
    Sub {
        arg1: usize,
        arg2: usize,
        arg3: usize,
    },
    Concat {
        arg1: usize,
        arg2: usize,
        arg3: usize,
    },
    Load {
        arg1: usize,
        arg2: usize,
    },
    LoadIntConst {
        arg1: usize,
        arg2: usize,
    },
    LoadFloatConst {
        arg1: usize,
        arg2: f64,
    },
    LoadStringConst {
        arg1: usize,
        arg2: Box<str>,
    },
    Move {
        arg1: usize,
        arg2: usize,
    },
    Store {
        arg1: usize,
        arg2: usize,
    },
    StoreIntConst {
        arg1: usize,
        arg2: usize,
    },
    StoreFloatConst {
        arg1: usize,
        arg2: f64,
    },
    StoreStringConst {
        arg1: usize,
        arg2: Box<str>,
    },
    Print {
        arg1: usize,
    },
    Halt,
}
