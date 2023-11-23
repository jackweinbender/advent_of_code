#! /usr/bin/env ruby
require_relative '../ruby_helpers'

input = $stdin.readlines(chomp: true)

def parse_input(string)
  arr = string.gsub(".", '').split(" ")
  subject   = arr[0]
  operator  = arr[2] == 'gain' ? 1 : -1
  value     = arr[3].to_i * operator
  object    = arr[-1]

  [subject, value, object]
end

tests 'parse input' do  
  assert(parse_input('Alice would gain 54 happiness units by sitting next to Bob.') == ["Alice", 54, "Bob"])
  assert(parse_input('Alice would lose 79 happiness units by sitting next to Carol.') == ["Alice", -79, "Carol"])
end

part_one = input
part_two = input

# puts "Part 1: #{part_one}"
# puts "Part 2: #{part_two}"
