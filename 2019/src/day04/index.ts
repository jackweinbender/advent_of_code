import { test } from "../utils/index"

const isAscending = (number) => {
  const arr = `${number}`.split('')
  for (let i = 1; i < arr.length; i++) {
    const j = i - 1
    if(arr[i] < arr[j]){
      return false
    }
  }
  return true
}

const hasDouble = (number) => {
  const arr = `${number}`.split('')
  for (let i = 1; i < arr.length; i++) {
    const j = i - 1
    if(arr[j] === arr[i]){
      return true
    }
  }
  return false
}

const exactlyDouble = (number) => {
  const arr = `${number}`.split('')
  const map = [0,0,0,0,0,0,0,0,0,0]
  arr.forEach( i => map[parseInt(i)] += 1 )
  return map.includes(2)
}

const input = [265275, 781584]

const goA = (input) => {
  const [start, stop] = input
  const range = []
  for (let i = start; i <= stop; i++) {
    range.push(i)
  }

  return range.filter(isAscending).filter(hasDouble).length
}

const goB = (input) => {
  const [start, stop] = input
  const range = []
  for (let i = start; i <= stop; i++) {
    range.push(i)
  }

  return range
    .filter(isAscending)
    .filter(hasDouble)
    .filter(exactlyDouble)
    .length
}

/* Tests */

test(hasDouble(1233), true)
test(hasDouble(1223), true)
test(hasDouble(1123), true)

test(hasDouble(1234), false)

test(isAscending(1111), true)
test(isAscending(1222), true)
test(isAscending(1234), true)

test(isAscending(4321), false)
test(isAscending(2341), false)
test(isAscending(2134), false)
test(isAscending(2314), false)

test(isAscending(111111) && hasDouble(111111), true)
test(isAscending(111123) && hasDouble(111123), true)
test(isAscending(122345) && hasDouble(122345), true)
test(isAscending(223450) && hasDouble(223450), false)
test(isAscending(123789) && hasDouble(123789), false)

test(exactlyDouble(112233), true)
test(exactlyDouble(123444), false)
test(exactlyDouble(111122), true)

/* Results */

console.time("Time")
const resultA = goA(input)
const resultB = goB(input)
console.timeEnd("Time")

console.log("Solution to part 1:", resultA)
console.log("Solution to part 2:", resultB)
