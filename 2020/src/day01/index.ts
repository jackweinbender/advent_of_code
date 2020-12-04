import { test, readInput } from "../utils/index";

const prepareInput = (rawInput: string): number[] => {
  return rawInput.split("\n").map((e) => parseInt(e.trim()));
};
const input = prepareInput(readInput());

const goA = (input: Array<number>) => {
  const TARGET = 2020;
  const [a, b] = pairThatSumsTo(input, TARGET);
  return a * b;
};

const pairThatSumsTo = (input: number[], target: number): [number, number] => {
  const s = new Set();

  for (var entry in input) {
    const given = input[entry];
    const inverse = target - given;

    if (s.has(inverse)) {
      return [given, inverse];
    } else {
      s.add(given);
    }
  }
  return null;
};

const goB = (input) => {
  const TARGET = 2020;
  // for each number, find a pair of numbers that equal it's inverse
  // If we find a pair, we return the product of the pair * given
  for (let entry in input) {
    const given = input[entry];
    const localTarget = TARGET - given;

    const pair = pairThatSumsTo(input, localTarget);

    if (pair) {
      const [a, b] = pair;
      return given * a * b;
    }
  }
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
