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
  return input.reduce(sumValid(policy_two), 0);
};

/* Tests */
const isBetween = (min: number, max: number) => (x: string) =>
  parseInt(x) >= min && parseInt(x) <= max;

const birthYearValidator = (x: string): boolean => isBetween(1920, 2002)(x);
const issueYearValidator = (x: string): boolean => isBetween(2010, 2020)(x);
const expYearValidator = (x: string): boolean => isBetween(2020, 2030)(x);
const heightValidator = (x: string): boolean => {
  const unit = x.slice(-2);
  const value = x.replace(unit, "");

  switch (unit) {
    case "cm":
      return isBetween(150, 193)(value);
      break;
    case "in":
      return isBetween(59, 76)(value);
      break;
    default:
      return false;
  }
};
const hairColorValidator = (x: string): boolean => {
  if (x[0] !== "#") return false;
  return x
    .slice(1)
    .split("")
    .every((x) =>
      [
        "0",
        "1",
        "2",
        "3",
        "4",
        "5",
        "6",
        "7",
        "8",
        "9",
        "a",
        "b",
        "c",
        "d",
        "e",
        "f",
      ].includes(x)
    );
};
const eyeColorValidator = (x: string): boolean =>
  ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].includes(x);
const passportIdValidator = (x: string): boolean =>
  x.length === 9 && !isNaN(parseInt(x));

const validate = (val: string, fn: Function) => (val ? fn(val) : false);

const policy_two: Map<string, Function> = new Map([
  ["byr", (x) => validate(x, birthYearValidator)],
  ["iyr", (x) => validate(x, issueYearValidator)],
  ["eyr", (x) => validate(x, expYearValidator)],
  ["hgt", (x) => validate(x, heightValidator)],
  ["hcl", (x) => validate(x, hairColorValidator)],
  ["ecl", (x) => validate(x, eyeColorValidator)],
  ["pid", (x) => validate(x, passportIdValidator)],
  ["cid", (x) => true],
]);

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

const testInputInvalid = `
eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007
`;

test(goB(prepareInput(testInputInvalid)), 0);

const testInputValid = `
pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022
`;

test(goB(prepareInput(testInputValid)), 3);

/* Results */

console.time("Time");
const resultA = goA(input);
const resultB = goB(input);
console.timeEnd("Time");

console.log("Solution to part 1:", resultA);
console.log("Solution to part 2:", resultB);
