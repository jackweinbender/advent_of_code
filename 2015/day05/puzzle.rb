require 'set'

def assert(boolean)
  message = boolean ? 'PASSED' : 'FAILED'
  puts message
end

BADDIES = Set.new(%w[ab cd pq xy])
VOWELS = Set.new(%w[a e i o u])

def nice_a?(string)
  # match = false
  has_double = false
  vowel_count = 0

  for idx in (1..string.length)
    curr_char = string[idx - 1]
    next_char = string[idx]

    return false if BADDIES.include?("#{curr_char}#{next_char}")

    has_double ||= (curr_char == next_char)
    vowel_count += 1 if VOWELS.include?(curr_char)
  end

  has_double && vowel_count >= 3
end

# Tests
puts '--- TESTS ---'
assert nice_a?('ugknbfddgicrmopn') == true
assert nice_a?('aaa') == true
assert nice_a?('jchzalrnumimnmhp') == false
assert nice_a?('haegwjzuvuyypxyu') == false
assert nice_a?('dvszwmarrgswjxmb') == false
puts '-------------'

input = File.readlines('input.txt', chomp: true)

part1 = input.filter_map { |word| nice_a?(word) }.count

puts "Part 1: #{part1}"
# puts "Part 2: #{part2}"
