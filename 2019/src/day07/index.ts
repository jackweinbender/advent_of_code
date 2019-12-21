import { test, readInput } from "../utils/index";
import { ThrustComputer } from "./lib/ThrustComputer";
import { Amplifier } from "./lib/Amplifier";

const prepareInput = (rawInput: string) => {
  return rawInput
    .trim()
    .split(",")
    .map(n => n.trim())
    .map(n => parseInt(n));
};

const input = prepareInput(readInput());

const getPermutations = (input: Array<number>) => {
  let result = [];
  const permute = (arr, m = []) => {
    if (arr.length === 0) {
      result.push(m);
    } else {
      for (let i = 0; i < arr.length; i++) {
        let curr = arr.slice();
        let next = curr.splice(i, 1);
        permute(curr.slice(), m.concat(next));
      }
    }
  };
  permute(input);
  return result;
};

const thrustFromConfig = (i: Array<number>, conf: Array<number>) => {
  const a = new Amplifier([...i]);
  const b = new Amplifier([...i]);
  const c = new Amplifier([...i]);
  const d = new Amplifier([...i]);
  const e = new Amplifier([...i]);

  const cpu = new ThrustComputer([a, b, c, d, e], conf);

  return cpu.computeThrust();
};

const feedbackThrustFromConfig = (i: Array<number>, conf: Array<number>) => {
  const a = new Amplifier([...i]);
  const b = new Amplifier([...i]);
  const c = new Amplifier([...i]);
  const d = new Amplifier([...i]);
  const e = new Amplifier([...i]);

  const cpu = new ThrustComputer([a, b, c, d, e], conf);

  return cpu.computeThrustWithFeedback();
};

const max = (acc, next) => {
  if (next > acc) {
    return next;
  } else {
    return acc;
  }
};

const goA = i => {
  const confs = getPermutations([0, 1, 2, 3, 4]);
  return confs.map(c => thrustFromConfig([...i], c)).reduce(max, 0);
};

const goB = i => {
  const confs = getPermutations([5, 6, 7, 8, 9]);
  return confs.map(c => feedbackThrustFromConfig([...i], c)).reduce(max, 0);
};

/* Tests */

const t1_input_raw = `
3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0
`;
const t1_input = prepareInput(t1_input_raw);
const t1 = thrustFromConfig([...t1_input], [4, 3, 2, 1, 0]);
test(t1, 43210);

const t2_input_raw = `
3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0
`;
const t2_input = prepareInput(t2_input_raw);
const t2 = thrustFromConfig([...t2_input], [0, 1, 2, 3, 4]);
test(t2, 54321);

const t3_input_raw = `
3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0
`;
const t3_input = prepareInput(t3_input_raw);
const t3 = thrustFromConfig([...t3_input], [1, 0, 4, 3, 2]);
test(t3, 65210);

test(goA(input), 92663);

const b1_input_raw = `
3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5
`;
const b1_input = prepareInput(b1_input_raw);
const b1 = feedbackThrustFromConfig([...b1_input], [9, 8, 7, 6, 5]);
test(b1, 139629729);

const b2_input_raw = `
3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10
`;
const b2_input = prepareInput(b2_input_raw);
const b2 = feedbackThrustFromConfig([...b2_input], [9, 7, 8, 5, 6]);
test(b2, 18216);

test(goB(input), 14365052);

/* Results */

console.time("Time");
const resultA = goA(input);
const resultB = goB(input);
console.timeEnd("Time");

console.log("Solution to part 1:", resultA);
console.log("Solution to part 2:", resultB);
