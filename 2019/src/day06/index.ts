import { test, readInput } from "../utils/index";

const prepareInput = (rawInput: string) => {
  return rawInput
    .trim()
    .split("\n")
    .map(o => o.trim().split(")"));
};

const input = prepareInput(readInput());

const buildMap = (acc: Object, next: Array<number>) => {
  const p = next[1];
  const orbits = next[0];
  acc[p] = orbits;
  return acc;
};

const countOrbits = (map: object, key: string) => {
  return getOrbits(map, key).length;
};

const getOrbits = (map: object, key: string) => {
  let orbs = [];
  while (key in map) {
    key = map[key];
    orbs = [key, ...orbs];
  }
  return orbs;
};

const goA = input => {
  const oMap: object = input.reduce(buildMap, {});
  let total = 0;

  Object.keys(oMap).forEach(e => {
    total += countOrbits(oMap, e);
  });

  return total;
};

const goB = input => {
  const oMap: object = input.reduce(buildMap, {});

  const orbs_you = new Set(getOrbits(oMap, "YOU"));
  const orbs_san = new Set(getOrbits(oMap, "SAN"));

  const difference = new Set(
    [...orbs_san, ...orbs_you].filter(x => !orbs_san.has(x) || !orbs_you.has(x))
  );

  return difference.size;
};

/* Tests */
const test_input_raw = `
COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
`;

const test_input = prepareInput(test_input_raw.trim());
// console.log(test_input);
test(goA(test_input), 42);

const test_input_raw2 = `
COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN
`;

const test_input2 = prepareInput(test_input_raw2.trim());
// console.log(test_input2);
test(goB(test_input2), 4);

/* Results */

console.time("Time");
const resultA = goA(input);
const resultB = goB(input);
console.timeEnd("Time");

console.log("Solution to part 1:", resultA);
console.log("Solution to part 2:", resultB);
