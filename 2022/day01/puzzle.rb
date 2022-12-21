input = File.readlines('input.txt', chomp: true)

elves = []
input.reduce(0) do |acc, nxt|
  if nxt.empty?
    elves << acc
    acc = 0
  else
    acc += nxt.to_i
  end
  acc
end

puts "Part 1: #{elves.max}"
puts "Part 2: #{elves.max(3).sum}"