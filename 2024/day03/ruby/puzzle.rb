#! /usr/bin/env ruby

input = $stdin.readlines(chomp: true)

part_one = input.join('')
  .scan(/mul\((\d{1,3}),(\d{1,3})\)/)
  .map do |match|
    match.map(&:to_i).reduce(1,&:*)
  end.sum

part_two = input.join('')
  .scan(/mul\((\d{1,3}),(\d{1,3})\)|(do)\(\)|(don\'t)\(\)/)
  .reduce([0, true]) do |acc, match|
    x, y, doo, dont = match
    if !doo.nil?
      acc[1] = true
      acc
    elsif !dont.nil?
      acc[1] = false
      acc
    elsif acc[1] == true
      acc[0] += x.to_i * y.to_i
      acc
    else
      acc
    end
  end
  .first

puts "Part 1: #{part_one}"
puts "Part 2: #{part_two}"
