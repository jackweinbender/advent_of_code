#! /usr/bin/env ruby
input = STDIN.read.split("\n")

require_relative './shapes'

parsed = input.map do |line|
  them, you = line.split(' ')
  [Shape.from(them), you]
end

part1 = parsed.map { |them, you| them.against(Shape.from(you)) }.sum
part2 = parsed.map { |them, you| them.against(Shape.for_outcome(them, you)) }.sum

puts "Part 1: #{part1}"
puts "Part 2: #{part2}"
