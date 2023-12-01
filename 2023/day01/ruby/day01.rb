#! /usr/bin/env ruby

input = $stdin.readlines(chomp: true)

part_one = input.map { |line|
  # wishing for an `is_numeric` method. Get all numbers.
  nums = line.split('').select{|ch| ch.to_i == 0 ? false : true}
  # This should cover the [7] == "77" case.
  "#{nums.first}#{nums.last}".to_i
}.sum


yuck = {
  "one" => "1",
  "two" => "2",
  "three" => "3",
  "four" => "4",
  "five" => "5",
  "six" => "6",
  "seven" => "7",
  "eight" => "8",
  "nine" => "9"
}

exp = /^(\d|one|two|three|four|five|six|seven|eight|nine)/

part_two = input.map { |line|
  numbers = []
  for idx in (0...line.length)
    match = line.slice(idx, line.length).match(exp)
    numbers << match[1] if match
  end
  [numbers.first, numbers.last]
}.map { |pair| "#{yuck[pair[0]] || pair[0]}#{yuck[pair[1]] || pair[1]}".to_i }.sum

puts "Part 1: #{part_one}"
puts "Part 2: #{part_two}"
