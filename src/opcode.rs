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
    Load {
        arg1: usize,
        arg2: usize,
    },
    LoadImm {
        arg1: usize,
        arg2: usize,
    },
    Move {
        arg1: usize,
        arg2: usize,
    },
    Store {
        arg1: usize,
        arg2: usize,
    },
    StoreImm {
        arg1: usize,
        arg2: usize,
    },
    Print {
        arg1: usize,
    },
    Halt,
}
