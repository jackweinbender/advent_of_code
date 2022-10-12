require_relative '../ruby_helpers'

# libcode A

# Tests A
tests 'Part A' do
  assert true
end

# libcode B

# Tests
tests 'Part B' do
  assert true
end

input = File.readlines('input.txt', chomp: true)

part1 = input.filter_map { |word| word }.count
part2 = input.filter_map { |word| word }.count

results do
  puts "Part 1: #{part1}".blue
  puts "Part 2: #{part2}".blue
end
