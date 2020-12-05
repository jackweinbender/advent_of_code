import { test, readInput } from "../utils/index";

const prepareInput = (rawInput: string) =>
  rawInput
    .split("\n\n")
    .map((l) => l.trim())
    .map(splitFields);
const splitFields = (block) =>
  block
    .split(/\s+/)
    .map((l) => l.trim())
    .map(splitKVP);
const splitKVP = (entry) => entry.split(":").map((l) => l.trim());

const input = prepareInput(readInput());

const requiredFields = ["ecl", "pid", "eyr", "hcl", "byr", "iyr", "hgt"];
const isValid = (req: string[], given: string[]): boolean =>
  req.every((f) => given.includes(f));
const sumValid = (req: string[]) => (acc, next) => {
  const valid = isValid(req, next);
  return valid ? (acc += 1) : acc;
};

const goA = (input) => {
  return input
    .map((p) => p.map((e) => e[0]))
    .reduce(sumValid(requiredFields), 0);
};

const goB = (input) => {
  return;
};

/* Tests */

test(
  isValid(requiredFields, [
    "ecl",
    "pid",
    "eyr",
    "hcl",
    "byr",
    "iyr",
    "cid",
    "hgt",
  ]),
  true
);

const testInput = `
ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in
`;

test(goA(prepareInput(testInput)), 2);

/* Results */

console.time("Time");
const resultA = goA(input);
const resultB = goB(input);
console.timeEnd("Time");

console.log("Solution to part 1:", resultA);
console.log("Solution to part 2:", resultB);
