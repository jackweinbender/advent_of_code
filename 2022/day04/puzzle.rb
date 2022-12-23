#! /usr/bin/env ruby

require 'set'

class Assignment
  attr_reader :range

  def self.from(string)
    start, stop = string.split('-').map(&:to_i)
    new(start, stop)
  end

  def initialize(start, stop)
    @range = Set.new(Range.new(start, stop))
  end
end

def overlapping?(a, b)
  a.range.subset?(b.range) || a.range.superset?(b.range)
end

input = $stdin.readlines(chomp: true)
              .map { |line| line.split(',').map { |r| Assignment.from(r) } }

part_one = input.select { |a, b| overlapping?(a, b) }.count
part_two = input

puts "Part 1: #{part_one}"
# puts "Part 2: #{part_two}"
