import { type } from "os";
import { test, readInput } from "../utils/index";

const prepareInput = (rawInput: string) =>
  rawInput
    .split("\n")
    .map((e) => e.trim())
    .map((e) => e.split(": "))
    .map(([l, r]) => [policyFromString(l), r]);

const input = prepareInput(readInput());

type Policy = {
  char: string;
  min: number;
  max: number;
};
type Password = string;

function policyFromString(input: string): Policy {
  const [mm, char] = input.split(" ");
  const [min, max] = mm.split("-").map((e) => parseInt(e));

  return { char, min, max };
}
const sum = (acc, next) => acc + next;
const sumValid = ([pol, pwd]) => (isValid(pwd, pol) ? 1 : 0);

function isValid(pwd: Password, policy: Policy): boolean {
  const charCount = pwd
    .split("")
    .map((c) => (c === policy.char ? 1 : 0))
    .reduce(sum, 0);

  return charCount <= policy.max && charCount >= policy.min;
}

const goA = (input) => {
  return input.map(sumValid).reduce(sum, 0);
};

const goB = (input) => {
  return;
};

/* Tests */

test(isValid("abcde", { min: 1, max: 3, char: "a" }), true);

/* Results */

console.time("Time");
const resultA = goA(input);
const resultB = goB(input);
console.timeEnd("Time");

console.log("Solution to part 1:", resultA);
console.log("Solution to part 2:", resultB);
