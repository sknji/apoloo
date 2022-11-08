pub(crate) enum FunctionType {
    TypeFunction,
    TypeScript,
}

pub(crate) struct Function {
    pub arity: i8,
    pub bytecodes: Bytecodes,
    pub name: String,
}
