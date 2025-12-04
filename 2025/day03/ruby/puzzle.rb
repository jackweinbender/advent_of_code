#! /usr/bin/env ruby
input = $stdin.readlines(chomp: true)

def maxJolts(arr, len=2)
  return "" if len <= 0
  return arr.join('') if arr.length == len

  head = arr[0..(arr.length - len)].max
  tail = arr[(arr.index(head) + 1)..-1]
  return "#{head}#{maxJolts(tail, len-1)}"
end

part_one = input.map{|line| maxJolts(line.split('')).to_i }.sum
part_two = input.map{|line| maxJolts(line.split(''), 12).to_i }.sum

puts "Part 1: #{part_one}"
puts "Part 2: #{part_two}"
