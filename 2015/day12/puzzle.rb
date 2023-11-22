#! /usr/bin/env ruby
require_relative '../ruby_helpers'
require 'json'

input = $stdin.readlines(chomp: true)

json = JSON.parse(input[0])

def handle_value(unknown, skip_red=false)
  return 0 if unknown.is_a? String
  return unknown if unknown.is_a? Numeric
  return unknown.map{ |v| handle_value(v, skip_red) }.sum if unknown.is_a? Array

  if unknown.is_a? Object
    return 0 if skip_red && unknown.values.include?('red')
    return unknown.values.map{ |v| handle_value(v, skip_red) }.sum
  end
end

tests 'basics' do
  assert(handle_value("test") == 0)
  assert(handle_value(1) == 1)
  assert(handle_value([1,2,3]) == 6)
  assert(handle_value({a: 1, b: 2, c: 3}) == 6)
end

tests 'examples' do
  assert(handle_value([1,2,3]) == 6)
  assert(handle_value({"a":2,"b":4}) == 6)

  assert(handle_value([[[3]]]) == 3)
  assert(handle_value({"a":{"b":4},"c":-1}) == 3)

  assert(handle_value({"a":[-1,1]}) == 0)
  assert(handle_value([-1,{"a":1}]) == 0)

  assert(handle_value([]) == 0)
  assert(handle_value({}) == 0)
end

part_one = handle_value(json)
part_two = handle_value(json, true)

puts "Part 1: #{part_one}"
puts "Part 2: #{part_two}"
