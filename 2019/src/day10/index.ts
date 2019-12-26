import { test, readInput } from "../utils/index";
import { Asteroid, Slope } from "./Asteroid";
import { PerformanceObserver } from "perf_hooks";
import { parse } from "querystring";
const prepareInput = (rawInput: string) => {
  return rawInput
    .trim()
    .split("\n")
    .map(row => row.trim().split(""))
    .map((row, ridx) => {
      return row.map((col, cidx) => {
        if (col === "#") {
          return new Asteroid(cidx, ridx);
        } else {
          return false;
        }
      });
    })
    .reduce((curr, next) => [...curr, ...next])
    .filter(a => a !== false);
};

const input = prepareInput(readInput());

const goA = input => {
  const ast_map = {};
  for (let i = 0; i < input.length; i++) {
    const ast = input[i];
    for (let j = 0; j < input.length; j++) {
      if (i === j) {
        continue;
      }
      const other = input[j];
      const location = input[i].location();
      const slope = ast.relativeSlopeTo(other).to_string();
      if (location in ast_map) {
        ast_map[location].add(slope);
      } else {
        ast_map[location] = new Set([slope]);
      }
    }
  }
  // console.log(ast_map);
  const position = Object.entries(ast_map)
    // .map((k:string, v: Set<string>) => s)
    .reduce(
      (curr: any, next: any) => {
        if (next[1].size > curr[1].size) {
          return next;
        } else {
          return curr;
        }
      },
      ["", new Set([])]
    );
  return `${position[1].size} @ ${position[0]}`;
};

const goB = input => {
  const base = new Asteroid(25, 31);
  const groupedTargets = input
    .filter(ast => base.distanceTo(ast) !== 0)
    .reduce((acc: any, next: Asteroid) => {
      const relSlope = base.relativeSlopeTo(next);
      const rad = relSlope.degrees;
      if (rad in acc) {
        acc[rad].push(next);
        acc[rad].sort((a, b) => {
          const da = base.distanceTo(a);
          const db = base.distanceTo(b);
          return da - db;
        });
      } else {
        acc[rad] = [next];
      }
      return acc;
    }, {});

  const rads = Object.keys(groupedTargets).sort(
    (a, b) => parseFloat(b) - parseFloat(a)
  );

  let idx = rads.indexOf("90");
  const output = [];
  while (output.length < 200) {
    const key = rads[idx];
    const ast = groupedTargets[key].pop();
    output.push(ast);

    idx = (idx + 1) % rads.length;
  }
  return output[199];
};

/* Tests */

const ast_a = new Asteroid(0, 0);
const ast_b = new Asteroid(1, 1);
test(ast_a.relativeSlopeTo(ast_b), new Slope(-1, 1));

const ast_c = new Asteroid(2, 2);
test(
  ast_a.relativeSlopeTo(ast_b).degrees,
  ast_a.relativeSlopeTo(ast_c).degrees
);
test(ast_b.relativeSlopeTo(ast_a), new Slope(1, -1));

const ast_d = new Asteroid(3, 0);
test(ast_b.relativeSlopeTo(ast_d), new Slope(1, 2));

test(new Slope(1, 2).to_string(), "1/2");

const t1 = `
.#..#
.....
#####
....#
...##
`;
const t1_input = prepareInput(t1);
test(goA(t1_input), `8 @ 3,4`);

const t2 = `
......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####
`;
const t2_input = prepareInput(t2);
test(goA(t2_input), `33 @ 5,8`);

const t3 = `
#.#...#.#.
.###....#.
.#....#...
##.#.#.#.#
....#.#.#.
.##..###.#
..#...##..
..##....##
......#...
.####.###.
`;
const t3_input = prepareInput(t3);
test(goA(t3_input), `35 @ 1,2`);

const t4 = `
.#..#..###
####.###.#
....###.#.
..###.##.#
##.##.#.#.
....###..#
..#.#..#.#
#..#.#.###
.##...##.#
.....#.#..
`;
const t4_input = prepareInput(t4);
test(goA(t4_input), `41 @ 6,3`);

const t5 = `
.#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##
`;
const t5_input = prepareInput(t5);
test(goA(t5_input), `210 @ 11,13`);

/* Results */

console.time("Time");
const resultA = goA(input);
const resultB = goB(input);
console.timeEnd("Time");

console.log("Solution to part 1:", resultA);
console.log("Solution to part 2:", resultB);
