require_relative '../ruby_helpers'

INPUT = File.readlines('input.txt', chomp: true)

str = '"basdab\"asdvasd\x45"'
total_length = str.length

def calc_mem_length(str)
  mem_length = 0
  skip = 0
  escape = false
  str.split("").each.with_index do |char, index|
    
    if skip > 0
      skip -= 1
      next
    end

    if char == 'x' && escape == true
      mem_length += 1
      skip = 2
      escape = false
      next
    end

    if char == '"'
      if escape == true
        mem_length += 1
        escape = false
      end
    elsif char == "\\"
      if escape == true
        mem_length += 1
        escape = false
      else
        escape = true
      end
    else
      mem_length += 1
    end

  end
  return mem_length
end

tests 'examples' do

  assert('""'.length == 2)
  assert(calc_mem_length('""') == 0)

  assert('"abc"'.length == 5)
  assert(calc_mem_length('"abc"') == 3)

  assert('"aaa\"aaa"'.length == 10)
  assert(calc_mem_length('"aaa\"aaa"') == 7)

  assert('"\x27"'.length == 6)
  assert(calc_mem_length('"\x27"') == 1)

  example_set = [
      '""',
      '"abc"',
      '"aaa\"aaa"',
      '"\x27"'
  ]

  assert(example_set.map { |str| str.length - calc_mem_length(str) }.sum == 12)

end

result = INPUT.map { |str| str.length - calc_mem_length(str) }.sum

puts "Part A: #{result}" 