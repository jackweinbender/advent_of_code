#! /usr/bin/env ruby


input = $stdin.readlines(chomp: true)
  .map{ |e| e.split(' ').map(&:strip).map(&:to_i)}
  .reduce({left: [], right: []}) do |acc, itr|
    acc[:left] << itr.first
    acc[:right] << itr.last
    acc
  end

input[:left] = input[:left].sort
input[:right] = input[:right].sort

part_one = input[:left].zip(input[:right]).reduce(0) do |acc, itr|
  acc + (itr.first - itr.last).abs
end

#
#

counts = input[:right].reduce(Hash.new(0)) do |acc, itr|
  acc[itr]+= 1
  acc
end

part_two = input[:left].reduce(0){ |acc, itr| acc + (counts[itr] * itr)}

puts "Part 1: #{part_one}"
puts "Part 2: #{part_two}"
