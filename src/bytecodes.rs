use crate::value::{Value, ValueRepr};

#[derive(Debug, Clone)]
pub struct Bytecodes {
    pub code_count: usize,
    pub code_capacity: usize,

    pub code: Vec<u8>,

    pub values_count: usize,
    pub values_capacity: usize,

    pub values: Vec<Value>,
}

impl Bytecodes {
    pub fn new() -> Bytecodes {
        Self {
            code_count: 0,
            code_capacity: 0,
            code: Vec::new(),
            values_count: 0,
            values_capacity: 0,
            values: Vec::new(),
        }
    }

    pub fn write(&mut self, byte: u8) -> usize {
        self.code.push(byte);
        self.code_count += 1;
        &self.code_count - 1
    }

    pub fn write2(&mut self, byte1: u8, byte2: u8) -> usize {
        self.write(byte1);
        self.write(byte2)
    }

    pub fn add_const_val(&mut self, val: f64) -> usize {
        self.add_const(Value(ValueRepr::Number(val)))
    }

    pub fn add_const(&mut self, value: Value) -> usize {
        self.values.push(value);
        self.values_count += 1;
        &self.values_count - 1
    }

    pub fn free(&mut self) {
        self.code.resize(0, 0);
        self.values.resize(0, Value::new());
        self.code_count = 0;
        self.code_capacity = 0;
        self.values_capacity = 0;
        self.values_count = 0;
    }
}

impl Default for Bytecodes {
    fn default() -> Bytecodes {
        Bytecodes::new()
    }
}
