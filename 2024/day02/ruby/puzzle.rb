#! /usr/bin/env ruby

input = $stdin.readlines(chomp: true).map { |e| e.split(' ').map(&:strip).map(&:to_i) }

def direction(left, right)
    return :asc if left < right
    return :desc
end

def gradual?(left, right)
  delta = (left - right).abs
  delta >= 1 && delta <= 3
end

def safe?(arr)
  dir = nil
  arr.each_cons(2).all? do |pair|
    left, right = pair
    return false unless gradual?(left, right)
    if dir.nil?
      dir = direction(left, right)
      true
    end
    return false unless dir == direction(left, right)
    true
  end
end

part_one = input.select { |e| safe?(e) }.length
part_two = input.select{ |report|
  subreports = report.map.with_index do |_, idx|
    r = report.dup
    r.delete_at(idx)
    safe?(r)
  end
  subreports.any?
}.length

puts "Part 1: #{part_one}"
puts "Part 2: #{part_two}"
