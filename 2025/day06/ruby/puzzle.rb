#! /usr/bin/env ruby


input = $stdin.readlines(chomp: true)

*heads, ops = input.map{|line| line.strip.split(/\s+/)}
grid = input.map{|line| line.split("")}.transpose

part_one = ops.each_with_index.map do |op, idx|
  case op
  when "+"
    heads.map{|line| line[idx] }.reduce(0){|acc,nxt| acc + nxt.to_i}
  when "*"
    heads.map{|line| line[idx] }.reduce(1){|acc,nxt| acc * nxt.to_i}
  end
end.sum

def part_two(grid)
  [*grid, Array.new(grid[0].length, " ")].reduce({total: 0, buffer: [], op: nil}) do |acc,row|
    if row.all?{|e| e.strip.empty? }
      raise if acc[:op] == nil
      # Do the maths
      acc[:total] += acc[:buffer].reduce(0){|acc,nxt| acc + nxt.to_i} if acc[:op] == "+"
      acc[:total] += acc[:buffer].reduce(1){|acc,nxt| acc * nxt.to_i} if acc[:op] == "*"
      # reset
      acc[:buffer] = []
      acc[:op] = nil
    else
      if row.last != " "
        acc[:op] = row.pop
      end
      acc[:buffer] << row.join('').to_i
    end
    acc
  end

end

puts "Part 1: #{part_one}"
puts "Part 2: #{part_two(grid)[:total]}"
