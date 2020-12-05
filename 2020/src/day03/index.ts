import { test, readInput } from "../utils/index";

const plantTrees = (g) => g.split("").map((t) => (t === "#" ? 1 : 0));
const prepareInput = (rawInput: string) => rawInput.split("\n").map(plantTrees);

const input = prepareInput(readInput());

type Point = {
  x: number;
  y: number;
};

type Slope = {
  rise: number;
  run: number;
};

const stepGenerator = function* (step: number, len: number) {
  let idx = 0;
  while (true) {
    yield idx;
    idx += step;
    idx %= len;
  }
};

const twoDStepGen = function* (
  yStep: number,
  yLen: number,
  xIdx: Generator<any, void, Point>
) {
  let yIdx = 0;

  while (yIdx < yLen) {
    yield { x: xIdx.next().value, y: yIdx };
    yIdx += yStep;
  }
};

const goA = (input) => {
  const idx = stepGenerator(3, input[0].length);

  return input.reduce((acc, next) => {
    return (acc += next[idx.next().value]);
  }, 0);
};

const goB = (input) => {
  let treeProduct = 1;
  const slopes: Slope[] = [
    { rise: 1, run: 1 },
    { rise: 1, run: 3 },
    { rise: 1, run: 5 },
    { rise: 1, run: 7 },
    { rise: 2, run: 1 },
  ];
  const lineLen = input[0].length;
  for (let i in slopes) {
    let xGen = stepGenerator(slopes[i].run, lineLen);
    let slopeGen = twoDStepGen(slopes[i].rise, input.length, xGen);
    let treeCount = 0;
    for (const s of slopeGen) {
      treeCount += input[s.y][s.x];
    }

    treeProduct *= treeCount;
  }

  return treeProduct;
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
