import { test, readInput } from "../utils/index"
import { Instruction } from "./instruction"

const prepareInput = (rawInput: string) => {
  let [red, green] = rawInput.trim()
    .split("\n").map( i => i.split(",").map( i => new Instruction(i) ))
}

const input = prepareInput(readInput())

const goA = (input) => {
  return
}

const goB = (input) => {
  return
}

/* Tests */

// test()

/* Results */

console.time("Time")
const resultA = goA(input)
const resultB = goB(input)
console.timeEnd("Time")

console.log("Solution to part 1:", resultA)
console.log("Solution to part 2:", resultB)
