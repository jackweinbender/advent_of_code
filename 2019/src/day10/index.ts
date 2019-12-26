import { test, readInput } from "../utils/index";
import { Asteroid, Slope } from "./Asteroid";
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
      const slope = ast.relativeSlopeTo(other);
      if (location in ast_map) {
        ast_map[location].add(slope);
      } else {
        ast_map[location] = new Set([slope]);
      }
    }
  }
  // console.log(ast_map);
  return Object.values(ast_map)
    .map((s: Set<string>) => s.size)
    .reduce((curr: number, next: number) => {
      if (next > curr) {
        return next;
      } else {
        return curr;
      }
    }, 0);
};

const goB = input => {
  return;
};

/* Tests */

const ast_a = new Asteroid(0, 0);
const ast_b = new Asteroid(1, 1);
test(ast_a.relativeSlopeTo(ast_b), new Slope(-1, 1).to_string());

const ast_c = new Asteroid(2, 2);
test(ast_a.relativeSlopeTo(ast_b), ast_a.relativeSlopeTo(ast_c));
test(ast_b.relativeSlopeTo(ast_a), new Slope(1, -1).to_string());

const ast_d = new Asteroid(3, 0);
test(ast_b.relativeSlopeTo(ast_d), new Slope(1, 2).to_string());

test(new Slope(1, 2).to_string(), "1/2");

const t1 = `
.#..#
.....
#####
....#
...##
`;
const t1_input = prepareInput(t1);
test(goA(t1_input), 8);

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
test(goA(t2_input), 33);

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
test(goA(t3_input), 35);

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
test(goA(t4_input), 41);

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
test(goA(t5_input), 210);

/* Results */

console.time("Time");
const resultA = goA(input);
const resultB = goB(input);
console.timeEnd("Time");

console.log("Solution to part 1:", resultA);
// console.log("Solution to part 2:", resultB);
