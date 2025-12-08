#! /usr/bin/env ruby
require 'set'

input = $stdin.read.split("\n\n")
top = input[0].split("\n")
  .map do |line|
    range = line.split("-").map(&:to_i)
    Range.new(*range)
  end
bottom = input[1].split("\n").map(&:to_i)

def count_fresh(top, bottom)
  bottom.map do |id|
    top.any?{ |range| range.include?(id) } ? 1 : 0
  end.sum
end

def fresh_range(top)
  l = top.sort{|a,b| a.first <=> b.first }
  .reduce([[], (0...0)]) do |acc,nxt|
    if nxt.overlap?(acc[1])
      min = [acc[1].min, nxt.min].min
      max = [acc[1].max, nxt.max].max
      acc[1] = (min..max)
    else
      acc[0] << acc[1]
      acc[1] = nxt
    end
    acc
  end
  l[0].map(&:size).sum + l[1].size
end

part_one = count_fresh(top, bottom)
part_two = fresh_range(top)

puts "Part 1: #{part_one}"
puts "Part 2: #{part_two}"
