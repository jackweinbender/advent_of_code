import { test, readInput } from "../utils/index"
import { Wire } from "./wire"
import { Panel } from "./panel"
import { Point } from "./point"

const prepareInput = (rawInput: string) => {
  let [red, green] = rawInput.trim().split("\n").map( i => i.trim() )
  return { red, green }
}

const input = prepareInput(readInput())

const closest_point = (a: Point, b: Point) => {
  if (a.distance() < b.distance()) { return a }
  else { return b }
}


const goA = (input) => {
  const red   = Wire.from_str(input.red)
  const green = Wire.from_str(input.green)

  const ixs = Panel.intersections(red, green)
  return [...ixs].reduce(closest_point).distance()
}

const goB = (input) => {
  const red   = Wire.from_str(input.red)
  const green = Wire.from_str(input.green)

  const ixs = Panel.intersections(red, green)
  const ixs_dists = [...ixs].map(ix => {
    return red.distance_to_point(ix) + green.distance_to_point(ix)
  })
  return Math.min(...ixs_dists)
}

/* Tests */

/** Part 1 **/
const a  = Wire.from_str("R8,U5,L5,D3")
const b = Wire.from_str("U7,R6,D4,L4")
const ab = Panel.intersections(a, b);
test([...ab].reduce(closest_point).distance(), 6)

const c  = Wire.from_str("R75,D30,R83,U83,L12,D49,R71,U7,L72")
const d = Wire.from_str("U62,R66,U55,R34,D71,R55,D58,R83")
const cd = Panel.intersections(c, d);
test([...cd].reduce(closest_point).distance(), 159)

const e  = Wire.from_str("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51")
const f = Wire.from_str("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7")
const ef = Panel.intersections(e, f);
test([...ef].reduce(closest_point).distance(), 135)

// Regression
test(goA(input), 1064)

/** Part 2 **/

const x  = Wire.from_str("R8,U5,L5,D3")
const y = Wire.from_str("U7,R6,D4,L4")
const xy = Panel.intersections(x, y);
const xs = [...xy].map(ix => {
  return x.distance_to_point(ix) + y.distance_to_point(ix)
})
test(Math.min(...xs), 30)

const g  = Wire.from_str("R75,D30,R83,U83,L12,D49,R71,U7,L72")
const h = Wire.from_str("U62,R66,U55,R34,D71,R55,D58,R83")
const gh = Panel.intersections(g, h);
const gs = [...gh].map(ix => {
  return g.distance_to_point(ix) + h.distance_to_point(ix)
})
test(Math.min(...gs), 610)


const i  = Wire.from_str("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51")
const j = Wire.from_str("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7")
const ij = Panel.intersections(i, j);
const is = [...ij].map(ix => {
  return i.distance_to_point(ix) + j.distance_to_point(ix)
})
test(Math.min(...is), 410)

/* Results */

console.time("Time")
const resultA = goA(input)
const resultB = goB(input)
console.timeEnd("Time")

console.log("Solution to part 1:", resultA)
console.log("Solution to part 2:", resultB)
