pub enum FunctionType {
    TypeFunction,
    TypeScript,
}

pub struct Function {
    pub arity: i8,
    pub bytecodes: Bytecodes,
    pub name: String,
}
