import { test, readInput } from "../utils/index";
import { Amplifier } from "./lib/Amplifier";
import { RelativeMode } from "./lib/Opcodes";

const prepareInput = (rawInput: string) => {
  const arr = rawInput
    .trim()
    .split(",")
    .map(n => n.trim())
    .map(n => parseInt(n));

  let obj = {};
  for (let i = 0; i < arr.length; i++) {
    obj[i] = arr[i];
  }
  return obj;
};

const input = prepareInput(readInput());

const goA = input => {
  const amp = new Amplifier({ ...input });
  return amp.compute([1]);
};

const goB = input => {
  const amp = new Amplifier({ ...input });
  return amp.compute([2]);
};

/* Tests */
const t1_raw = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99";
const t1 = prepareInput(t1_raw);
const t1_amp = new Amplifier({ ...t1 });
test(t1_amp.compute([]), [
  109,
  1,
  204,
  -1,
  1001,
  100,
  1,
  100,
  1008,
  100,
  16,
  101,
  1006,
  101,
  0,
  99
]);

const t2_raw = "1102,34915192,34915192,7,4,7,99,0";
const t2 = prepareInput(t2_raw);
const t2_amp = new Amplifier({ ...t2 });
test(t2_amp.compute([])[0].toString().length, 16);

const t3_raw = "104,1125899906842624,99";
const t3 = prepareInput(t3_raw);
const t3_amp = new Amplifier({ ...t3 });
test(t3_amp.compute([])[0], 1125899906842624);

test(goA(input), [2870072642]);
test(goB(input), [58534]);

/* Results */

console.time("Time");
const resultA = goA(input);
const resultB = goB(input);
console.timeEnd("Time");

console.log("Solution to part 1:", resultA[0]);
console.log("Solution to part 2:", resultB[0]);
