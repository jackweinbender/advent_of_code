#! /usr/bin/env ruby


input = $stdin.readlines(chomp: true).map { |e| e.split(' ').map(&:strip).map(&:to_i) }

def set_dir(left, right)
  return :asc if left < right
  return :desc
end

# The levels are either all increasing or all decreasing.
# Any two adjacent levels differ by at least one and at most three.
def consistent?(left, right, dir)
  if dir == :asc
    left < right
  else
    left > right
  end
end

def gradual?(left, right, dir)
  delta = (left - right).abs
  delta >= 1 && delta <= 3
end

def safe?(arr, dampened=false)
  idx = 0
  dir = nil
  while idx < arr.length - 1
    jdx = idx+1
    left, right = arr[idx], arr[jdx]
    if dir.nil?
      dir = set_dir(left, right)
    end

    ok = consistent?(left, right, dir) && gradual?(left, right, dir)

    unless ok
      if dampened
        jdx += 1
        right = arr[jdx]
        return false unless consistent?(left, right, dir) && gradual?(left, right, dir)
      else
        return false
      end
    end

    idx = jdx
  end

  true
end

part_one = input.map{ |e| safe?(e) ? 1 : 0 }.sum
part_two = input.map{ |e| safe?(e, true) ? 1 : 0 }.sum

puts "Part 1: #{part_one}"
puts "Part 2: #{part_two}"
