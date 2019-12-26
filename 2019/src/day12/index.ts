import { test, readInput } from "../utils/index";
import { Moon, System } from "./Moon";

const prepareInput = (rawInput: string) => {
  return rawInput
    .trim()
    .split("\n")
    .map(l => l.trim())
    .map(l => Moon.from(l));
};

const input = prepareInput(readInput());
const input2 = prepareInput(readInput());

const goA = input => {
  const s = new System(input);
  s.step(1000);
  return s.total_energy();
};

const goB = input => {
  const s = new System(input);
  return s.steps_til_repeat();
};

/* Tests */

const g = new Moon({ x: 3, y: 3, z: 3 });
const c = new Moon({ x: 5, y: 5, z: 5 });
System.compose(g, g);
test(g.velocity, { x: 0, y: 0, z: 0 });

System.compose(g, c);
test(g.velocity, { x: 1, y: 1, z: 1 });
test(c.velocity, { x: -1, y: -1, z: -1 });

g.step();
c.step();
test(g.position, { x: 4, y: 4, z: 4 });
test(c.position, { x: 4, y: 4, z: 4 });

System.compose(g, c);
test(g.velocity, { x: 1, y: 1, z: 1 });
test(c.velocity, { x: -1, y: -1, z: -1 });

g.step();
c.step();
test(g.position, { x: 5, y: 5, z: 5 });
test(c.position, { x: 3, y: 3, z: 3 });

const t1_raw = `
<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>
`;
const t1 = prepareInput(t1_raw);
const t1_s = new System(t1);
t1_s.updateVelocities();
test(t1_s.moons[0].velocity, { x: 3, y: -1, z: -1 });
t1_s.updatePositions();
test(t1_s.moons[0].position, { x: 2, y: -1, z: 1 });

t1_s.step();
test(t1_s.moons[0].velocity, { x: 3, y: -2, z: -2 });
test(t1_s.moons[0].position, { x: 5, y: -3, z: -1 });

const t2_raw = `
<x=-8, y=-10, z=0>
<x=5, y=5, z=10>
<x=2, y=-7, z=3>
<x=9, y=-8, z=-3>
`;
const t2 = prepareInput(t2_raw);
const t2_s = new System(t2);

t2_s.step(100);
test(t2_s.total_energy(), 1940);

const t3_raw = `
<x= -1, y=  0, z=  2>
<x=  2, y=-10, z= -7>
<x=  4, y= -8, z=  8>
<x=  3, y=  5, z= -1>
`;

const t3 = prepareInput(t3_raw);
const t3_s = new System(t3);
test(t3_s.steps_til_repeat(), 2772);

const t4_raw = `
<x=-8, y=-10, z=0>
<x=5, y=5, z=10>
<x=2, y=-7, z=3>
<x=9, y=-8, z=-3>
`;

const t4 = prepareInput(t4_raw);
const t4_s = new System(t4);
test(t4_s.steps_til_repeat(), 4686774924);

/* Results */

console.time("Time");
const resultA = goA([...input]);
const resultB = goB([...input2]);
console.timeEnd("Time");

console.log("Solution to part 1:", resultA);
console.log("Solution to part 2:", resultB);

// 391586508119752
