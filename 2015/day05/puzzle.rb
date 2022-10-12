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
puts '--- PART A TESTS ---'
assert nice_a?('ugknbfddgicrmopn') == true
assert nice_a?('aaa') == true
assert nice_a?('jchzalrnumimnmhp') == false
assert nice_a?('haegwjzuvuyypxyu') == false
assert nice_a?('dvszwmarrgswjxmb') == false
puts '--------------------'

# It contains a pair of any two letters that appears at least twice in the string without overlapping, like xyxy (xy) or aabcdefgaa (aa), but not like aaa (aa, but it overlaps).
# It contains at least one letter which repeats with exactly one letter between them, like xyx, abcdefeghi (efe), or even aaa.
def nice_b?(string)
  doubledouble = false
  xyx = false

  pairs = Set.new
  i = 1
  while doubledouble == false && i <= string.length
    pair = "#{string[i - 1]}#{string[i]}"
    doubledouble = pairs.include?(pair) unless doubledouble == true
    pairs.add(pair)

    i += 1 if string[i] == string[i - 1] && string[i] == string[i + 1]
    i += 1
  end

  for idx in (1..string.length)
    curr_char = string[idx - 1]
    next_char = string[idx]
    thrd_char = string[idx + 1]

    xyx ||= (curr_char == thrd_char)

  end
  doubledouble && xyx
end

# Tests
puts '--- PART B TESTS ---'
assert nice_b?('qjhvhtzxzqqjkmpb') == true
assert nice_b?('xxyxx') == true
assert nice_b?('uurcxstgmygtbstg') == false
assert nice_b?('ieodomkazucvgmuy') == false
assert nice_b?('aaa') == false
puts '--------------------'

input = File.readlines('input.txt', chomp: true)

part1 = input.filter_map { |word| nice_a?(word) }.count
part2 = input.filter_map { |word| nice_b?(word) }.count

puts "Part 1: #{part1}"
puts "Part 2: #{part2}"
