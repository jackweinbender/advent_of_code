import { test, readInput } from "../utils/index";

const prepareInput = (rawInput: string): number[] => {
  return rawInput.split("\n").map((e) => parseInt(e.trim()));
};
const input = prepareInput(readInput());

const TARGET = 2020;

const goA = (input: Array<number>) => {
  const s = new Set();

  for (var entry in input) {
    const given = input[entry];
    const inverse = TARGET - given;

    if (s.has(inverse)) {
      return given * inverse;
    } else {
      s.add(given);
    }
  }
};

const goB = (input) => {
  return;
};

/* Tests */

// test()

/* Results */

console.time("Time");
const resultA = goA(input);
const resultB = goB(input);
console.timeEnd("Time");

console.log("Solution to part 1:", resultA);
console.log("Solution to part 2:", resultB);
