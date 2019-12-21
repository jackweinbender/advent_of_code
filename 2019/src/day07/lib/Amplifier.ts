import { OpCodeFactory } from "./Opcodes";

type InstructionSet = Array<number>;

export class Amplifier {
  idx: number;
  instructions: InstructionSet;
  constructor(instructions: InstructionSet) {
    this.instructions = instructions;
    this.idx = 0;
  }
  compute(inputs: Array<number>) {
    let output = [];

    while (this.instructions[this.idx] != 99 && output.length < 1) {
      let code = OpCodeFactory.fromCode(
        this.instructions[this.idx],
        inputs,
        output
      );
      this.instructions = code.applyCodeAtIndex(this.instructions, this.idx);

      inputs = code.getInputs();
      output = code.getOutput();

      this.idx = code.nextIndex(this.instructions, this.idx);
    }
    if (output.length > 0) {
      return output[0];
    } else {
      return false;
    }
  }
}
