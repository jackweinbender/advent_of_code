#! /usr/bin/env ruby


input = $stdin.readlines(chomp: true).map{|lines| lines.split('')}

def check(pos, arr, limit=4)
  kernel = [1,0,-1].product([1,0,-1])
  t = kernel.map do |p|
    y = p[0] + pos[0]
    x = p[1] + pos[1]
    if p == [0,0] || y < 0 || x < 0
      0
    elsif arr.dig(y, x) == "@" || arr.dig(y, x) == "x"
      1
    else
      0
    end
  end
  t.sum
end

def part_one(input)
  input.each_with_index.map do |row,idx|
    row.each_with_index.map do |col, jdx|
      if input[idx][jdx] == "@"
        val = check([idx, jdx], input)
        input[idx][jdx] = "x" if val < 4
        val < 4 ? 1 : 0
      else
        0
      end
    end.sum
  end.sum
end


def part_two(input)
  output = 0
  while true
    part_one(input)
    removed = input.map{|r| r.map{|i| i == "x" ? 1 : 0}.sum}.sum

    break if removed == 0

    output += removed
    input = input.map{|r| r.map{|i| i == "x" ? "." : i}}
  end
  output
end

puts "Part 1: #{part_one(input.map(&:clone))}"
puts "Part 2: #{part_two(input.map(&:clone))}"
