import { test, readInput } from "../utils/index";

const prepareInput = (rawInput: string) =>
  rawInput
    .trim()
    .split("\n")
    .map((line) => [line.slice(0, 7), line.slice(7)]);

const input = prepareInput(readInput());

const binaryMapper = (low: string, high: string) => (char: string) => {
  if (char === low) return "0";
  if (char === high) return "1";
  throw new Error(`Invalid char ${char}`);
};

const fbMapper = binaryMapper("F", "B");
const lrMapper = binaryMapper("L", "R");

const toBinaryString = (str: string, fn: Function): string =>
  str
    .split("")
    .map((x) => fn(x))
    .join("");

type Ticket = [string, string];
type ID = number;

const seatID = (ticket: Ticket): ID => {
  const row = parseInt(ticket[0], 2);
  const col = parseInt(ticket[1], 2);
  return 8 * row + col;
};

const goA = (input) => {
  const seatIds = input.map(([r, c]) => {
    const row = toBinaryString(r, fbMapper);
    const col = toBinaryString(c, lrMapper);
    return seatID([row, col]);
  });
  return Math.max(...seatIds);
};
const goB = (input) => {
  const seatIds = input.map(([r, c]) => {
    const row = toBinaryString(r, fbMapper);
    const col = toBinaryString(c, lrMapper);
    return seatID([row, col]);
  });
  const min = Math.min(...seatIds);
  const max = Math.max(...seatIds);

  const missing = [];

  for (let i = min; i < max; i++) {
    if (!seatIds.includes(i)) missing.push(i);
  }
  return missing;
};

/* Tests */

test(toBinaryString("FFF", fbMapper), "000");
test(toBinaryString("BBB", fbMapper), "111");
test(toBinaryString("LLL", lrMapper), "000");
test(toBinaryString("RRR", lrMapper), "111");

/* Results */

console.time("Time");
const resultA = goA(input);
const resultB = goB(input);
console.timeEnd("Time");

console.log("Solution to part 1:", resultA);
console.log("Solution to part 2:", resultB);
