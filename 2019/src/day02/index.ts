import { test, readInput } from "../utils/index";

const prepareInput = (rawInput: string) => {
  return rawInput
    .split(",")
    .map(n => n.trim())
    .map(n => parseInt(n));
};

const input = prepareInput(readInput());

const process = (codes: Array<number>) => {
  let idx = 0;

  while (codes[idx] != 99) {
    codes = updateCodes(codes, idx);
    idx += 4
  }

  return codes
};

const updateCodes = (codes: Array<number>, opcodeIndex: number) => {
  const code = codes[opcodeIndex];
  if (code === 1) {
    return add(codes, opcodeIndex);
  } else if (code === 2) {
    return multiply(codes, opcodeIndex);
  } else {
    throw "Bad Opcode!";
  }
};

const add = (codes: Array<number>, idx: number) => {
  const leftIdx = codes[idx + 1];
  const rightIdx = codes[idx + 2];
  const outIdx = codes[idx + 3];

  codes[outIdx] = codes[leftIdx] + codes[rightIdx];
  return codes;
};

const multiply = (codes: Array<number>, idx: number) => {
  const leftIdx = codes[idx + 1];
  const rightIdx = codes[idx + 2];
  const outIdx = codes[idx + 3];

  codes[outIdx] = codes[leftIdx] * codes[rightIdx];
  return codes;
};

const goA = codes => {
  codes[1] = 12
  codes[2] = 2

  codes = process(codes)
  return codes[0]
};

const goB = codes => {
  let noun = 0;
  let verb = 0;

  for (let noun = 0; noun < 100; noun++) {
    for (let verb = 0; verb < 100; verb++) {
      let cs = [...codes]
      cs[1] = noun
      cs[2] = verb

      cs = process(cs)

      if(cs[0] === 19690720){ return (100 * noun) + verb }
    }
  }
};

/* Tests */

test(process([1, 0, 0, 0, 99]), [2, 0, 0, 0, 99]);
test(process([2, 3, 0, 3, 99]), [2, 3, 0, 6, 99]);
test(process([2, 4, 4, 5, 99, 0]), [22, 4, 4, 5, 99, 9801]);
test(process([1, 1, 1, 4, 99, 5, 6, 0, 99]), [30, 1, 1, 4, 2, 5, 6, 0, 99]);

/* Results */

console.time("Time");
const resultA = goA([...input]);
const resultB = goB([...input]);
console.timeEnd("Time");

console.log("Solution to part 1:", resultA);
console.log("Solution to part 2:", resultB);
