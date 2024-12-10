#! /usr/bin/env ruby


input = $stdin.readlines(chomp: true).map{|line| line.strip.split('')}

part_one = input
part_two = input

def guard_init(map)
  map.each_with_index do |row, idx|
    row.each_with_index do |col, jdx|
      if map[idx][jdx] == '^'
        return {
          pos: [idx, jdx],
          dir: '^'
        }
      end
    end
  end
  throw "Can't find guard!?"
end

def on_map(pos, map)
  row, col = pos
  return false if row < 0 || row >= map.length
  return false if col < 0 || col >= map[0].length
  return true
end

def peek(guard, map)
  row, col = guard[:pos]
  case guard[:dir]
  when '^'
    row -= 1
  when 'v'
    row += 1
  when '<'
    col -= 1
  when '>'
    col += 1
  end
  [row, col]
end

def turn(guard)
  case guard[:dir]
  when '^'
    guard[:dir] = '>'
  when '>'
    guard[:dir] = 'v'
  when 'v'
    guard[:dir] = '<'
  when '<'
    guard[:dir] = '^'
  end
end

def blocked?(pos, map)
  row, col = pos
  map[row][col] == "#"
end

def main(map, guard)
  loop_detected = false

  visited = Set.new
  visited.add(guard.clone)

  while !loop_detected
    next_pos = peek(guard, map)

    break if !on_map(next_pos, map)

    if blocked?(next_pos, map)
      turn(guard)
      visited.add(guard.clone)
      next
    end

    guard[:pos] = next_pos.clone

    loop_detected = visited.include?(guard)
    visited.add(guard.clone)
  end
  return [visited, loop_detected]
end

guard = guard_init(input)
path, _ = main(input.map(&:dup), guard.clone)

# def part2(path, map)
#   positions = Set.new
#   path.each do |guard|
#     next_pos = peek(guard, map)

#     if !on_map(next_pos, map) || blocked?(next_pos, map)
#       next
#     end

#     candidate = map.map(&:dup)

#     row, col = next_pos
#     candidate[row][col] = "#"

#     _, loop_detected = main(candidate, guard.clone)
#     positions.add(next_pos) if loop_detected
#   end
#   positions
# end

def bruteforce(input)
  g = guard_init(input)
  loops = 0
  input.each_with_index do |r, row|
    puts "\n---"
    puts "Row #{row}:"
    r.each_with_index do |_, col|
      print "."
      next if [row, col] == g[:pos]
      next if input[row][col] == "#"

      map = input.map(&:dup)
      map[row][col] = "#"

      guard = g.clone
      _, loop_detected = main(map, guard)
      loops += 1 if loop_detected
    end
  end
  return loops
end

# part_two = part2(path, input)

puts "Part 1: #{path.map {|e| e[:pos]}.uniq.length}"
puts "Part 2: #{bruteforce(input)}"
