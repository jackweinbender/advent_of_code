import { OpCodeFactory } from "./Opcodes";

type InstructionSet = object;

export class Amplifier {
  idx: number;
  instructions: InstructionSet;
  offset: number;
  constructor(instructions: InstructionSet) {
    this.instructions = instructions;
    this.idx = 0;
    this.offset = 0;
  }
  compute(inputs: Array<number>) {
    let output = [];

    while (this.instructions[this.idx] != 99) {
      let code = OpCodeFactory.fromCode(
        this.instructions[this.idx],
        inputs,
        output,
        this.offset
      );
      // console.log({ code, idx: this.idx });
      this.instructions = code.applyCodeAtIndex(this.instructions, this.idx);

      inputs = code.getInputs();
      output = code.getOutput();

      this.offset = code.getOffset();
      if (this.offset === undefined) {
        console.log({ code, index: this.idx });
        throw "";
      }
      this.idx = code.nextIndex(this.instructions, this.idx, this.offset);
    }
    if (output.length > 0) {
      return output;
    } else {
      return false;
    }
  }
}
