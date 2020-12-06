import { test, readInput } from "../utils/index";

// Input Prep

const prepareInput = (rawInput: string) =>
  rawInput
    .split("\n\n")
    .map((l) => l.trim())
    .map(splitFields);
const splitFields = (block) =>
  new Map(
    block
      .split(/\s+/)
      .map((l) => l.trim())
      .map(splitKVP)
  );
const splitKVP = (entry) => entry.split(":").map((l) => l.trim());
const input = prepareInput(readInput());

// Business Logic
const isValid = (
  req: Map<string, Function>,
  given: Map<string, string>
): boolean => {
  const reqFields = Array.from(req.keys());

  return reqFields.every((f) => {
    const validatorFn = req.get(f);
    const value = given.get(f);
    return validatorFn(value);
  });
};

const sumValid = (req: Map<string, Function>) => (acc, next) =>
  isValid(req, next) ? (acc += 1) : acc;

// Part A Policy
const policy_one: Map<string, Function> = new Map([
  ["byr", (x) => (x ? true : false)],
  ["iyr", (x) => (x ? true : false)],
  ["eyr", (x) => (x ? true : false)],
  ["hgt", (x) => (x ? true : false)],
  ["hcl", (x) => (x ? true : false)],
  ["ecl", (x) => (x ? true : false)],
  ["pid", (x) => (x ? true : false)],
  ["cid", (x) => true],
]);

const goA = (input) => {
  return input.reduce(sumValid(policy_one), 0);
};

// Part B Policy

const goB = (input) => {
  return;
};

/* Tests */

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
