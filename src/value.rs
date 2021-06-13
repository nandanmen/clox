pub type Value = f32;

pub fn print_value(value: Value) {
  print!("{}", value);
}

pub struct ValueArray {
  pub values: Vec<Value>,
}

impl ValueArray {
  pub fn new() -> ValueArray {
    ValueArray { values: Vec::new() }
  }
  pub fn push(&mut self, value: Value) -> usize {
    self.values.push(value);
    self.values.len() - 1
  }
}
