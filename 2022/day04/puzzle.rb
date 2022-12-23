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

def full_overlap?(a, b)
  a.range.subset?(b.range) || a.range.superset?(b.range)
end

def overlaps?(a, b)
  a.range.intersect?(b.range)
end

input = $stdin.readlines(chomp: true)
              .map { |line| line.split(',').map { |r| Assignment.from(r) } }

part_one = input.select { |a, b| full_overlap?(a, b) }.count
part_two = input.select { |a, b| overlaps?(a, b) }.count

puts "Part 1: #{part_one}"
puts "Part 2: #{part_two}"
