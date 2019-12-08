import { test, readInput } from "../utils/index";

const prepareInput = (rawInput: string) => {
  return rawInput
    .split("\n")
    .map(l => l.trim())
    .map(l => parseInt(l));
};

const input = prepareInput(readInput());

const goA = input => {
  return input
    .map(module => fuelForMass(module))
    .reduce((sum, mass) => {
      return sum + mass;
    }, 0);
};

const goB = input => {
  return input
    .map(module => fuelForTotalMass(module))
    .reduce((sum, mass) => {
      return sum + mass;
    }, 0);
};

const fuelForMass = (mass: number) => {
  const fuel = Math.floor(mass / 3) - 2;
  if(fuel > 0){ return fuel }
  else { return 0 } 
};

const fuelForTotalMass = (mass: number, total: number = 0) => {
  if (mass <= 2) { return total }
  else {
    const addedFuelMass = fuelForMass(mass)
    return fuelForTotalMass(addedFuelMass, total + addedFuelMass)
  }
}

/* Tests */

test(fuelForMass(12), 2);
test(fuelForMass(14), 2);
test(fuelForMass(1969), 654);
test(fuelForMass(100756), 33583);

test(fuelForTotalMass(1969), 966);
test(fuelForTotalMass(100756), 50346);

/* Results */

console.time("Time");
const resultA = goA(input);
const resultB = goB(input);
console.timeEnd("Time");

console.log("Solution to part 1:", resultA);
console.log("Solution to part 2:", resultB);
