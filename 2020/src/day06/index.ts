import { test, readInput } from "../utils/index";

const prepareInput = (rawInput: string) =>
  rawInput
    .trim()
    .split("\n\n")
    .map((group) => group.split("\n"));

const input = prepareInput(readInput());

const sum = (a, b) => a + b;

const union = (set: Set<string>, other: Set<string>): Set<string> =>
  new Set([...set, ...other]);

const intersection = (set: Set<string>, other: Set<string>): Set<string> => {
  const intersection: Set<string> = new Set();
  for (let el of other) {
    if (set.has(el)) intersection.add(el);
  }
  return intersection;
};

const goA = (input) => {
  return input
    .map((group) => group.map((e) => new Set(e)).reduce(union).size)
    .reduce(sum);
};

const goB = (input) => {
  return input
    .map((group) => group.map((e) => new Set(e)).reduce(intersection).size)
    .reduce(sum);
};

/* Tests */
const test_input = `
abc

a
b
c

ab
ac

a
a
a
a

b
`;
test(goA(prepareInput(test_input)), 11);
test(goB(prepareInput(test_input)), 6);

/* Results */

console.time("Time");
const resultA = goA(input);
const resultB = goB(input);
console.timeEnd("Time");

console.log("Solution to part 1:", resultA);
console.log("Solution to part 2:", resultB);
