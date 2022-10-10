require 'set'

input = File.read('input.txt').strip.split('')

def offset(dir)
  return [0, 1] if dir == '^'
  return [0, -1] if dir == 'v'
  return [-1, 0] if dir == '<'
  return [1, 0] if dir == '>'

  raise ArgumentError
end

def move(pos, offset)
  pos[0] += offset[0]
  pos[1] += offset[1]

  pos
end

pos = [0, 0]
set = Set.new([pos])

input.each do |dir|
  pos = move(pos, offset(dir))
  set.add(pos)
end

santa = [0, 0]
robo_santa = [0, 0]
set2 = Set.new([santa.to_s])

input.each_with_index do |dir, idx|
  mover = idx.even? ? santa : robo_santa
  mover = move(mover, offset(dir))
  set2.add(mover.to_s)
end

puts "Part 1: #{set.length}"
puts "Part 2: #{set2.length}"
