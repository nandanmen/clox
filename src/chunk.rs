use crate::value::*;

#[derive(Debug, Clone, Copy)]
pub enum OpCode {
  LoadConstant,
  Return,
}

#[derive(Clone, Copy)]
pub enum Instruction {
  OpCode(OpCode),
  Argument(usize),
}

pub struct Chunk {
  // TODO: Make this private and figure out a loop operation
  pub code: Vec<Instruction>,
  pub constants: ValueArray,
  pub lines: Vec<u32>,
}

impl Chunk {
  pub fn init() -> Chunk {
    Chunk {
      code: Vec::new(),
      constants: ValueArray::new(),
      lines: Vec::new(),
    }
  }
  pub fn add_instruction(&mut self, data: Instruction, line: u32) {
    self.code.push(data);
    self.lines.push(line);
  }
  pub fn add_constant(&mut self, constant: Value) -> usize {
    self.constants.push(constant)
  }
}
