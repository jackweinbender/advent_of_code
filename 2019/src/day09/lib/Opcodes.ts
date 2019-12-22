type ParamCode = string;
type Index = number;
type Signature = Array<ParamMode>;

interface OpCode {
  params: Signature;
  applyCodeAtIndex(arr, idx: Index): Array<number>;
  nextIndex(arr, idx: Index, offset: number): Index;
  getInputs(): Array<number>;
  getOutput(): Array<number>;
}

export class OpCodeFactory {
  static fromCode(
    code: number,
    inputs: Array<number>,
    output: Array<number>,
    offset: number
  ) {
    const noramlizedCode = OpCodeFactory.normalizeCode(code);
    const type: string = OpCodeFactory.getCodeType(noramlizedCode);
    const signature: Signature = OpCodeFactory.getCodeParams(noramlizedCode);

    switch (type) {
      case "01":
        return new Add(signature, inputs, output, offset);
      case "02":
        return new Multiply(signature, inputs, output, offset);
      case "03":
        return new Input(signature[0], inputs, output, offset);
      case "04":
        return new Output(signature[0], inputs, output, offset);
      case "05":
        return new JumpIfTrue(signature.slice(0, 2), inputs, output, offset);
      case "06":
        return new JumpIfFalse(signature.slice(0, 2), inputs, output, offset);
      case "07":
        return new LessThan(signature, inputs, output, offset);
      case "08":
        return new Equals(signature, inputs, output, offset);
      case "09":
        return new UpdateRel(signature[0], inputs, output, offset);
      // case "99":
      //   return new Halt();
      default:
        throw "Unknown OpCode Type";
    }
  }
  static getCodeType(str: string) {
    return str.slice(-2);
  }
  static getCodeParams(str: string) {
    return str
      .slice(0, str.length - 2)
      .split("")
      .map(c => ParamModeFactory.fromCode(c))
      .reverse();
  }
  static normalizeCode(code: number) {
    return code.toString().padStart(5, "0");
  }
}

export class ParamModeFactory {
  static fromCode(code: ParamCode) {
    switch (code) {
      case "0":
        return new PositionMode();
      case "1":
        return new ImmediateMode();
      case "2":
        return new RelativeMode();
      default:
        throw "Unknown Param Code";
    }
  }
}

interface ParamMode {
  getFromIndex(arr, idx: Index, offset: number): number;
}

class PositionMode implements ParamMode {
  getFromIndex(arr, idx: Index, offset: number) {
    return arr[idx];
  }
}

class ImmediateMode implements ParamMode {
  getFromIndex(arr, idx: Index, offset: number) {
    return idx;
  }
}

export class RelativeMode implements ParamMode {
  getFromIndex(arr, idx: Index, offset: number) {
    // console.log({ idx, offset, arr_at: arr[idx], returns: arr[idx] + offset });
    return arr[idx] + offset;
  }
}

class OpCode {
  codeLen: number;
  outputBuffer: Array<number>;
  input: Array<number>;
  offset: number;
  nextIndex(arr, idx: Index, offset: number) {
    return idx + this.codeLen;
  }
  getInputs() {
    return this.input;
  }
  getOutput() {
    return this.outputBuffer;
  }
  getOffset() {
    return this.offset;
  }
}

export class Add extends OpCode implements OpCode {
  codeLen = 4;
  left: ParamMode;
  right: ParamMode;
  output: ParamMode;
  constructor(
    signature: Signature,
    input: Array<number>,
    output: Array<number>,
    offset: number
  ) {
    super();
    this.input = input;
    this.outputBuffer = output;
    this.offset = offset;
    [this.left, this.right, this.output] = signature;
  }
  applyCodeAtIndex(arr, idx: Index) {
    const param_l = idx + 1;
    const param_r = idx + 2;
    const param_o = idx + 3;

    const leftIdx = this.left.getFromIndex(arr, param_l, this.offset);
    const rightIdx = this.right.getFromIndex(arr, param_r, this.offset);
    const outIdx = this.output.getFromIndex(arr, param_o, this.offset);

    const l = arr[leftIdx] || 0;
    const r = arr[rightIdx] || 0;

    arr[outIdx] = l + r;

    return arr;
  }
}

export class Multiply extends OpCode implements OpCode {
  codeLen = 4;
  left: ParamMode;
  right: ParamMode;
  output: ParamMode;
  constructor(
    signature: Signature,
    input: Array<number>,
    output: Array<number>,
    offset: number
  ) {
    super();
    this.input = input;
    this.outputBuffer = output;
    this.offset = offset;
    [this.left, this.right, this.output] = signature;
  }
  applyCodeAtIndex(arr, idx: Index) {
    const param_l = idx + 1;
    const param_r = idx + 2;
    const param_o = idx + 3;

    const leftIdx = this.left.getFromIndex(arr, param_l, this.offset);
    const rightIdx = this.right.getFromIndex(arr, param_r, this.offset);
    const outIdx = this.output.getFromIndex(arr, param_o, this.offset);

    const l = arr[leftIdx] || 0;
    const r = arr[rightIdx] || 0;

    arr[outIdx] = l * r;

    return arr;
  }
}
export class Input extends OpCode implements OpCode {
  codeLen = 2;
  outputMode: ParamMode;
  constructor(
    mode: ParamMode,
    input: Array<number>,
    output: Array<number>,
    offset: number
  ) {
    super();
    this.input = input;
    this.outputBuffer = output;
    this.outputMode = mode;
    this.offset = offset;
  }
  applyCodeAtIndex(arr, idx: Index) {
    const outIdx = this.outputMode.getFromIndex(arr, idx + 1, this.offset);
    arr[outIdx] = this.input.shift();
    return arr;
  }
}
export class Output extends OpCode implements OpCode {
  codeLen = 2;
  outputMode: ParamMode;
  constructor(
    mode: ParamMode,
    input: Array<number>,
    output: Array<number>,
    offset: number
  ) {
    super();
    this.input = input;
    this.outputBuffer = output;
    this.outputMode = mode;
    this.offset = offset;
  }
  applyCodeAtIndex(arr, idx: Index) {
    const outIdx = this.outputMode.getFromIndex(arr, idx + 1, this.offset);
    const o = arr[outIdx];
    this.outputBuffer.push(o);
    return arr;
  }
}

export class JumpIfTrue extends OpCode implements OpCode {
  codeLen = 3;
  left: ParamMode;
  right: ParamMode;

  constructor(
    modes: Signature,
    input: Array<number>,
    output: Array<number>,
    offset: number
  ) {
    super();
    this.input = input;
    this.outputBuffer = output;
    this.offset = offset;
    [this.left, this.right] = modes;
  }
  applyCodeAtIndex(arr, idx: Index) {
    return arr;
  }
  nextIndex(arr, idx: Index, offset: number) {
    const left = this.left.getFromIndex(arr, idx + 1, offset);
    const right = this.right.getFromIndex(arr, idx + 2, offset);
    if (arr[left] === 0) {
      return idx + this.codeLen;
    } else {
      return arr[right];
    }
  }
}
export class JumpIfFalse extends OpCode implements OpCode {
  codeLen = 3;
  left: ParamMode;
  right: ParamMode;
  constructor(
    modes: Signature,
    input: Array<number>,
    output: Array<number>,
    offset: number
  ) {
    super();
    this.input = input;
    this.outputBuffer = output;
    this.offset = offset;
    [this.left, this.right] = modes;
  }
  applyCodeAtIndex(arr, idx: Index) {
    return arr;
  }
  nextIndex(arr, idx: Index, offset: number) {
    const left = this.left.getFromIndex(arr, idx + 1, offset);
    const right = this.right.getFromIndex(arr, idx + 2, offset);
    if (arr[left] === 0) {
      return arr[right];
    } else {
      return idx + this.codeLen;
    }
  }
}
export class LessThan extends OpCode implements OpCode {
  codeLen = 4;
  left: ParamMode;
  right: ParamMode;
  output: ParamMode;
  constructor(
    signature: Signature,
    input: Array<number>,
    output: Array<number>,
    offset: number
  ) {
    super();
    this.input = input;
    this.outputBuffer = output;
    this.offset = offset;
    [this.left, this.right, this.output] = signature;
  }
  applyCodeAtIndex(arr, idx: Index) {
    const param_l = idx + 1;
    const param_r = idx + 2;
    const param_o = idx + 3;

    const leftIdx = this.left.getFromIndex(arr, param_l, this.offset);
    const rightIdx = this.right.getFromIndex(arr, param_r, this.offset);
    const outIdx = this.output.getFromIndex(arr, param_o, this.offset);

    const l = arr[leftIdx] || 0;
    const r = arr[rightIdx] || 0;

    if (l < r) {
      arr[outIdx] = 1;
    } else {
      arr[outIdx] = 0;
    }

    return arr;
  }
}
export class Equals extends OpCode implements OpCode {
  codeLen = 4;
  left: ParamMode;
  right: ParamMode;
  output: ParamMode;
  constructor(
    signature: Signature,
    input: Array<number>,
    output: Array<number>,
    offset: number
  ) {
    super();
    this.input = input;
    this.outputBuffer = output;
    this.offset = offset;
    [this.left, this.right, this.output] = signature;
  }
  applyCodeAtIndex(arr, idx: Index) {
    const param_l = idx + 1;
    const param_r = idx + 2;
    const param_o = idx + 3;

    const leftIdx = this.left.getFromIndex(arr, param_l, this.offset);
    const rightIdx = this.right.getFromIndex(arr, param_r, this.offset);
    const outIdx = this.output.getFromIndex(arr, param_o, this.offset);

    const l = arr[leftIdx] || 0;
    const r = arr[rightIdx] || 0;
    if (l === r) {
      arr[outIdx] = 1;
    } else {
      arr[outIdx] = 0;
    }

    return arr;
  }
}
export class UpdateRel extends OpCode implements OpCode {
  codeLen = 2;
  outputMode: ParamMode;
  constructor(
    mode: ParamMode,
    input: Array<number>,
    output: Array<number>,
    offset: number
  ) {
    super();
    this.input = input;
    this.outputBuffer = output;
    this.outputMode = mode;
    this.offset = offset;
  }
  applyCodeAtIndex(arr, idx: Index) {
    const readIdx = this.outputMode.getFromIndex(arr, idx + 1, this.offset);
    this.offset = this.offset + (arr[readIdx] || 0);
    return arr;
  }
}
