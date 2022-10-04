use crate::value::Value;

#[derive(Debug)]
pub struct Bytecodes {
    pub(crate) code_count: usize,
    pub(crate) code_capacity: usize,

    pub(crate) code: Vec<u8>,

    pub(crate) values_count: usize,
    pub(crate) values_capacity: usize,

    pub(crate) values: Vec<Value>,
}

impl Bytecodes {
    pub(crate) fn new() -> Bytecodes {
        Self {
            code_count: 0,
            code_capacity: 0,
            code: Vec::with_capacity(0),
            values_count: 0,
            values_capacity: 0,
            values: Vec::new(),
        }
    }

    pub(crate) fn write(&mut self, byte: u8) -> usize {
        self.code.push(byte);
        self.code_count += 1;
        return self.code_count - 1;
    }

    pub(crate) fn add_const_val(&mut self, val: f64) -> usize {
        self.add_const(Value::new(val))
    }

    pub(crate) fn add_const(&mut self, value: Value) -> usize {
        self.values.push(value);
        self.values_count += 1;
        return self.values_count - 1;
    }

    pub(crate) fn free(&mut self) {
        self.code.resize(0, 0);
        self.values.resize(0, Value::new(0.0));
        self.code_count = 0;
        self.code_capacity = 0;
        self.values_capacity = 0;
        self.values_count = 0;
    }
}