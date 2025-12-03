#! /usr/bin/env ruby


input = $stdin.readlines(chomp: true)
  .map{|l| l.split(",").map{|r| r.split("-").map(&:to_i)}.map{|nums| Range.new(*nums)}}.first

def valid?(number)
  return false if number.length.odd?
  original = number.split('')
  original == original.rotate(number.length.div(2))
end

def validd?(number)
  original = number.split('')
  (1..number.length.div(2)).each do |num|
    next unless number.length % num == 0
    return true if original.rotate(num) == original
  end
  false
end

part_one = input.reduce(0) do |acc, nxt|
  nxt.each do |val|
    acc += val if valid?(val.to_s)
  end
  acc
end
part_two = input.reduce(0) do |acc, nxt|
  nxt.each do |val|
    acc += val if validd?(val.to_s)
  end
  acc
end



puts "Part 1: #{part_one}"
puts "Part 2: #{part_two}"
