import { test, readInput } from "../utils/index";

import { OpCodeFactory, ParamModeFactory, Add, Multiply } from "./lib/OpCodes";

const prepareInput = (rawInput: string) => {
  return rawInput
    .split(",")
    .map(n => n.trim())
    .map(n => parseInt(n));
};

const input = prepareInput(readInput());

const process = (codes: Array<number>, input: number) => {
  let idx = 0;
  console.log(`---BEGIN TESTS---`);
  while (codes[idx] != 99) {
    // console.log({ idx, value: codes[idx] });
    let code = OpCodeFactory.fromCode(codes[idx], input);
    // console.log({ code, idx });
    codes = code.applyCodeAtIndex(codes, idx);
    idx = code.nextIndex(codes, idx);
    // console.log({ idx });
  }
  console.log(`---HALT TESTS---`);
};

const goA = input => {
  process([...input], 1);
};

const goB = input => {
  process([...input], 5);
};

/* Tests */

test(OpCodeFactory.normalizeCode(1), "00001");
test(OpCodeFactory.normalizeCode(10001), "10001");

test(OpCodeFactory.getCodeType("12345"), "45");
test(OpCodeFactory.getCodeParams("10000"), [
  ParamModeFactory.fromCode("0"),
  ParamModeFactory.fromCode("0"),
  ParamModeFactory.fromCode("1")
]);

const t_add = [1, 0, 0, 0, 99];
const p_add = [
  ParamModeFactory.fromCode("0"),
  ParamModeFactory.fromCode("0"),
  ParamModeFactory.fromCode("0")
];
const add = new Add(p_add);

test(add.applyCodeAtIndex(t_add, 0), [2, 0, 0, 0, 99]);
test(add.nextIndex(t_add, 0), 4);

const t_mult = [2, 3, 0, 3, 99];
const p_mult = [
  ParamModeFactory.fromCode("0"),
  ParamModeFactory.fromCode("0"),
  ParamModeFactory.fromCode("0")
];
const mult = new Multiply(p_mult);

test(mult.applyCodeAtIndex(t_mult, 0), [2, 3, 0, 6, 99]);
test(mult.nextIndex(t_mult, 0), 4);

const posit_mult = [1002, 4, 3, 4, 33];
const m_code = OpCodeFactory.fromCode(1002);
test(m_code.applyCodeAtIndex(posit_mult, 0), [1002, 4, 3, 4, 99]);

const i_input = [3, 0, 4, 0, 99];
const i_code = OpCodeFactory.fromCode(3);
const step_1 = i_code.applyCodeAtIndex(i_input, 0);
test(step_1, [1, 0, 4, 0, 99]);

// const o_code = OpCodeFactory.fromCode(4);
// const step_2 = o_code.applyCodeAtIndex(step_1, 2);
// test(step_1, step_2);

let t = OpCodeFactory.fromCode(5);
let t_arr = [5, 3, 0, 1];
test(t.nextIndex(t_arr, 0), 5);

let t2 = OpCodeFactory.fromCode(5);
let t2_arr = [5, 3, 999, 0];
test(t2.nextIndex(t2_arr, 0), 3);

let f = OpCodeFactory.fromCode(6);
let f_arr = [6, 3, 0, 0];
test(f.nextIndex(f_arr, 0), 6);

let f2 = OpCodeFactory.fromCode(6);
let f2_arr = [6, 3, 999, 1];
test(f2.nextIndex(f2_arr, 0), 3);

const lt = OpCodeFactory.fromCode(7);
const lt_arr = [7, 3, 0, 1];
test(lt.applyCodeAtIndex(lt_arr, 0), [7, 1, 0, 1]);

const lt2 = OpCodeFactory.fromCode(7);
const lt2_arr = [7, 0, 3, 0];
test(lt2.applyCodeAtIndex(lt2_arr, 0), [0, 0, 3, 0]);

const eq = OpCodeFactory.fromCode(8);
const eq_arr = [8, 0, 0, 0];
test(eq.applyCodeAtIndex(eq_arr, 0), [1, 0, 0, 0]);

const eq2 = OpCodeFactory.fromCode(8);
const eq2_arr = [8, 0, 1, 0];
test(eq2.applyCodeAtIndex(eq2_arr, 0), [0, 0, 1, 0]);

/* Results */

console.time("Time");
const resultA = goA(input);
const resultB = goB(input);
console.timeEnd("Time");

console.log("Solution to part 1:", 14522484);
console.log("Solution to part 2:", 4655956);
