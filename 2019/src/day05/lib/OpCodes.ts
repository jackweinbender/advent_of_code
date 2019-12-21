type ParamCode = string;
type Index = number;
type Signature = Array<ParamMode>;
interface OpCode {
  params: Signature;
  applyCodeAtIndex(arr: Array<number>, idx: Index): Array<number>;
  nextIndex(arr: Array<number>, idx: Index): Index;
}

export class OpCodeFactory {
  static fromCode(code: number, input: number = 1) {
    const noramlizedCode = OpCodeFactory.normalizeCode(code);
    const type: string = OpCodeFactory.getCodeType(noramlizedCode);
    const signature: Signature = OpCodeFactory.getCodeParams(noramlizedCode);

    switch (type) {
      case "01":
        return new Add(signature);
      case "02":
        return new Multiply(signature);
      case "03":
        return new Input(input);
      case "04":
        return new Output(signature[0]);
      case "05":
        return new JumpIfTrue(signature.slice(0, 2));
      case "06":
        return new JumpIfFalse(signature.slice(0, 2));
      case "07":
        return new LessThan(signature);
      case "08":
        return new Equals(signature);
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
      default:
        throw "Unknown Param Code";
    }
  }
}

interface ParamMode {
  getFromIndex(arr: Array<number>, idx: Index): number;
}

class PositionMode implements ParamMode {
  getFromIndex(arr: Array<number>, idx: Index) {
    return arr[idx];
  }
}

class ImmediateMode implements ParamMode {
  getFromIndex(arr: Array<number>, idx: Index) {
    return idx;
  }
}

class OpCode {
  codeLen: number;
  nextIndex(arr: Array<number>, idx: Index) {
    return idx + this.codeLen;
  }
}

export class Add extends OpCode implements OpCode {
  codeLen = 4;
  left: ParamMode;
  right: ParamMode;
  output: ParamMode;
  constructor(signature: Signature) {
    super();
    [this.left, this.right, this.output] = signature;
  }
  applyCodeAtIndex(arr: Array<number>, idx: Index) {
    const param_l = idx + 1;
    const param_r = idx + 2;
    const param_o = idx + 3;

    const leftIdx = this.left.getFromIndex(arr, param_l);
    const rightIdx = this.right.getFromIndex(arr, param_r);
    const outIdx = arr[param_o]; // Ouput is never in Immediate mode

    arr[outIdx] = arr[leftIdx] + arr[rightIdx];
    return arr;
  }
}

export class Multiply extends OpCode implements OpCode {
  codeLen = 4;
  left: ParamMode;
  right: ParamMode;
  output: ParamMode;
  constructor(signature: Signature) {
    super();
    [this.left, this.right, this.output] = signature;
  }
  applyCodeAtIndex(arr: Array<number>, idx: Index) {
    const param_l = idx + 1;
    const param_r = idx + 2;
    const param_o = idx + 3;

    const leftIdx = this.left.getFromIndex(arr, param_l);
    const rightIdx = this.right.getFromIndex(arr, param_r);
    const outIdx = arr[param_o]; // Ouput is never in Immediate mode

    arr[outIdx] = arr[leftIdx] * arr[rightIdx];
    return arr;
  }
}
export class Input extends OpCode implements OpCode {
  codeLen = 2;
  input: number;
  constructor(input: number) {
    super();
    this.input = input;
  }
  applyCodeAtIndex(arr: Array<number>, idx: Index) {
    const outIdx = arr[idx + 1];

    arr[outIdx] = this.input;

    return arr;
  }
}
export class Output extends OpCode implements OpCode {
  codeLen = 2;
  outputMode: ParamMode;
  constructor(mode: ParamMode) {
    super();
    this.outputMode = mode;
  }
  applyCodeAtIndex(arr: Array<number>, idx: Index) {
    const outIdx = this.outputMode.getFromIndex(arr, idx + 1);
    console.log(`Process terminated with code ${arr[outIdx]}`);
    return arr;
  }
}

export class JumpIfTrue extends OpCode implements OpCode {
  codeLen = 3;
  left: ParamMode;
  right: ParamMode;
  constructor(modes: Signature) {
    super();
    [this.left, this.right] = modes;
  }
  applyCodeAtIndex(arr: Array<number>, idx: Index) {
    return arr;
  }
  nextIndex(arr: Array<number>, idx: Index) {
    const left = this.left.getFromIndex(arr, idx + 1);
    const right = this.right.getFromIndex(arr, idx + 2);
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
  constructor(modes: Signature) {
    super();
    [this.left, this.right] = modes;
  }
  applyCodeAtIndex(arr: Array<number>, idx: Index) {
    return arr;
  }
  nextIndex(arr: Array<number>, idx: Index) {
    const left = this.left.getFromIndex(arr, idx + 1);
    const right = this.right.getFromIndex(arr, idx + 2);
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
  constructor(signature: Signature) {
    super();
    [this.left, this.right, this.output] = signature;
  }
  applyCodeAtIndex(arr: Array<number>, idx: Index) {
    const param_l = idx + 1;
    const param_r = idx + 2;
    const param_o = idx + 3;

    const leftIdx = this.left.getFromIndex(arr, param_l);
    const rightIdx = this.right.getFromIndex(arr, param_r);
    const outIdx = arr[param_o]; // Ouput is never in Immediate mode

    if (arr[leftIdx] < arr[rightIdx]) {
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
  constructor(signature: Signature) {
    super();
    [this.left, this.right, this.output] = signature;
  }
  applyCodeAtIndex(arr: Array<number>, idx: Index) {
    const param_l = idx + 1;
    const param_r = idx + 2;
    const param_o = idx + 3;

    const leftIdx = this.left.getFromIndex(arr, param_l);
    const rightIdx = this.right.getFromIndex(arr, param_r);
    const outIdx = arr[param_o]; // Ouput is never in Immediate mode

    if (arr[leftIdx] === arr[rightIdx]) {
      arr[outIdx] = 1;
    } else {
      arr[outIdx] = 0;
    }

    return arr;
  }
}
