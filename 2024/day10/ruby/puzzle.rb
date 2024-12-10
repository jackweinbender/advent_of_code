#! /usr/bin/env ruby

input = $stdin.readlines(chomp: true).map{|row| row.split('').map(&:to_i)}

map = {}
trailheads = []
input.each_with_index do |row, rdx|
  row.each_with_index do |col, cdx|
    map["#{rdx},#{cdx}"] = input[rdx][cdx]
    trailheads << [rdx,cdx] if input[rdx][cdx] == 0
  end
end

def adjascent(row, col, input)
  [[1,0], [-1,0], [0,-1], [0,1]].filter_map do |pos|
    pos[0] += row
    pos[1] += col
    next if pos[0] < 0 || pos[0] >= input.length
    next if pos[1] < 0 || pos[1] >= input[0].length
    pos
  end
end

def ratings(map, trailheads, input)
  routes = 0
  dests = 0
  trailheads.each do |trail|
    queue = [trail]
    destinations = []
    while queue.length > 0
      row, col = queue.pop()
      if input[row][col] == 9
        destinations << [row, col]
        next
      end
      adj = adjascent(row, col, input)
      adj.each do |pos|
        r, c = pos
        queue << pos if input[r][c] == input[row][col] + 1
      end
    end
    routes += destinations.length
    dests += destinations.uniq.length
  end
  [dests, routes]
end

part_one, part_two = ratings(map, trailheads, input)

puts "Part 1: #{part_one}"
puts "Part 2: #{part_two}"
