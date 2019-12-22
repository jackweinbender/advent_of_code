import { test, readInput } from "../utils/index";
import { Image } from "./Image";

const prepareInput = (rawInput: string) => {
  let input = rawInput
    .trim()
    .split("")
    .map(d => parseInt(d));
  return Image.from_input(input, 25, 6);
};

const input = prepareInput(readInput());

const goA = image => {
  return image.twosByOnes();
};

const goB = image => {
  image.display();
};

/* Tests */

const t1 = Image.from_input([1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2], 3, 2);
test(t1.twosByOnes(), 1);
test(goA(input), 2210);

const t2 = Image.from_input(
  [0, 2, 2, 2, 1, 1, 2, 2, 2, 2, 1, 2, 0, 0, 0, 0],
  2,
  2
);
test(t2.getVisible().join(""), "0110");
test(
  input.getVisible().join(""),
  "011000110011110011001111010010100101000010010100001000010000111001000011100100001011010000101101000010010100101000010010100000110001110111100111011110"
);
/* Results */

console.time("Time");
const resultA = goA(input);
console.log("Solution to part 1:", resultA);
console.log("Solution to part 2:", "");
const resultB = goB(input);
console.timeEnd("Time");
