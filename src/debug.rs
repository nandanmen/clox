use crate::chunk::*;
use crate::value::*;

pub fn disassemble_chunk(chunk: &Chunk, name: &str) {
  println!("== {} ==", name);

  let mut offset = 0;
  while offset < chunk.code.len() {
    offset = disassemble_instruction(chunk, offset);
  }
}

pub fn disassemble_instruction(chunk: &Chunk, offset: usize) -> usize {
  print!("{} ", offset);

  if offset > 0 && chunk.lines[offset] == chunk.lines[offset - 1] {
    print!("  | ");
  } else {
    print!("{} ", chunk.lines[offset]);
  }

  let instruction = chunk.code[offset];
  match instruction {
    Instruction::OpCode(code) => match code {
      OpCode::Return => simple_instruction("OpReturn", offset),
      OpCode::LoadConstant => constant_instruction("OpLoadConstant", chunk, offset),
    },
    Instruction::Argument(arg) => {
      println!("Argument or unknown OpCode: {}", arg);
      offset + 1
    }
  }
}

fn constant_instruction(name: &str, chunk: &Chunk, offset: usize) -> usize {
  let next_bytecode = chunk.code[offset + 1];

  match next_bytecode {
    Instruction::OpCode(code) => panic!("Unexpected OpCode {:?}", code),
    Instruction::Argument(const_address) => {
      print!("{} {} ", name, const_address);

      let value = chunk.constants.values[const_address];
      print_value(value);

      println!("");
    }
  }

  offset + 2
}

fn simple_instruction(name: &str, offset: usize) -> usize {
  println!("{}", name);
  offset + 1
}
