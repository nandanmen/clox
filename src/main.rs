mod chunk;
mod debug;
mod value;

use chunk::Chunk;
use chunk::Instruction;
use chunk::OpCode;

fn main() {
  let mut chunk = Chunk::init();

  let const_address = chunk.add_constant(1.2);

  // add temporary line numbers
  chunk.add_instruction(Instruction::OpCode(OpCode::LoadConstant), 321);
  chunk.add_instruction(Instruction::Argument(const_address), 321);

  chunk.add_instruction(Instruction::OpCode(OpCode::Return), 321);

  debug::disassemble_chunk(&chunk, "test chunk");
}
