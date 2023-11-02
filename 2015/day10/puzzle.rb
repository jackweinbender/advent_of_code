require_relative '../ruby_helpers'

def say(arr)
  "#{arr.length}#{arr.first}"
end

def look(arr)
  arr.reduce([]) { |acc, nxt|
  current_grouping = acc&.last
  current_number = current_grouping&.last

  if nxt == current_number
    current_grouping << nxt
  else
    acc << [nxt]
  end

  acc
}

end


def look_and_say(input, iterations=1)
  return input if iterations.zero?

  result = look(input.split('')).map{|g| say(g)}.join('')
  look_and_say(result, iterations - 1)
end

tests 'Tests' do
  assert( look_and_say('1', 5) == '312211')
  assert( look_and_say('1') == '11')
  assert( look_and_say('11') == '21')
  assert( look_and_say('21') == '1211')
  assert( look_and_say('1211') == '111221')
  assert( look_and_say('111221') == '312211')

  assert( look('111222333'.split('')) == [['1','1','1'],['2','2','2'],['3','3','3']])

  assert( say(["1"]) == "11" )
  assert( say(["1", "1"]) == "21" )
  assert( say(["1", "1", "1"]) == "31" )
end


puts "Part 1: #{look_and_say('1321131112', 40).length}"
puts "Part 2: #{look_and_say('1321131112', 50).length}"
