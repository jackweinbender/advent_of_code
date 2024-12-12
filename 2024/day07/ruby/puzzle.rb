#! /usr/bin/env ruby


input = $stdin.readlines(chomp: true)
  .map{|line| line.split(":")
  .map {|e| e.split(" ").map{|e| e.strip().to_i } } }

part_one = input.filter do |eq|
  result = eq[1].reduce([]) do |acc, nxt|
    acc << nxt and next acc if acc.empty?
    acc.flat_map do |n|
      [nxt*n, nxt+n]
    end
  end
  result.any? {|val| val == eq[0][0] }
end

part_two = input.filter do |eq|
  result = eq[1].reduce([]) do |acc, nxt|
    acc << nxt and next acc if acc.empty?
    acc.flat_map do |n|
      [nxt*n, nxt+n, "#{n}#{nxt}".to_i]
    end
  end
  result.any? {|val| val == eq[0][0] }
end

puts "Part 1: #{ part_one.map{|eq| eq[0][0]}.sum }"
puts "Part 2: #{ part_two.map{|eq| eq[0][0]}.sum }"
