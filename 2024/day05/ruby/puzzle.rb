#! /usr/bin/env ruby


input = $stdin.read.split("\n\n")

rules = input[0].strip.split("\n").map{ |str| str.split("|").map{|page| page.to_i }}
pagesets = input[1].strip.split("\n").map{ |str| str.split(",").map{|page| page.to_i }}

valid_sets, invalid_sets = pagesets.partition do |pageset|
  indexes = {}
  rules.all? do |rule|
    indexes[rule[0]] = pageset.find_index(rule[0]) unless indexes.include?(rule[0])
    indexes[rule[1]] = pageset.find_index(rule[1]) unless indexes.include?(rule[1])

    # Ignore rules about pages we don't have
    if indexes[rule[0]].nil? || indexes[rule[1]].nil?
      true
    else
      indexes[rule[0]] < indexes[rule[1]]
    end
  end
end



sum_center = lambda do |sum, set|
  center = (set.length - 1) / 2
  sum += set[center]
end

part_one = valid_sets

part_two = invalid_sets.map do |set|
  set.sort do |a, b|
    # There should only ever be one of these, so there's no point in
    # memoizing the result here.
    rule = rules.find{ |e| e.include?(a) && e.include?(b)}
    # Since we're finding the center, asc/desc doesn't matter
    rule.find_index(a)<=>rule.find_index(b)
  end
end


puts "Part 1: #{part_one.reduce(0, &sum_center)}"
puts "Part 2: #{part_two.reduce(0, &sum_center)}"
