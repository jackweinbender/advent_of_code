#! /usr/bin/env ruby
require 'set'

input = STDIN.readlines(chomp: true)

def partition(line)
  center = line.length / 2
  # Left is exclusive of the center
  left = line[0...center]
  # Left is inclusive of the center
  right = line[center..]
  [left, right]
end

def intersection(array)
  array.reduce { |acc, nxt| acc & nxt }
end

def prioritize(char)
  if char == char.upcase
    char.ord - 38
  else
    char.ord - 96
  end
end

part_one = input.map { |pack| pack.split('') }
                .map { |line| partition(line) }
                .map { |pack| intersection(pack) }
                .map { |err| prioritize(*err) }.sum

part_two = input.map { |pack| pack.split('') }
                .each_with_object([]) do |nxt, acc|
                  if acc.empty? || acc.last.length == 3
                    acc << [nxt]
                  else
                    acc.last << nxt
                  end
                end
                .map { |group| intersection(group) }
                .map { |badge| prioritize(*badge) }.sum

puts "Part 1: #{part_one}"
puts "Part 2: #{part_two}"