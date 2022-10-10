require 'digest'

input = File.read('input.txt').strip

# part 1
hash = Digest::MD5.new
index = 0

while hash.hexdigest[0..4] != '00000'
  hash.reset
  index += 1
  p index if index % 1000 == 0
  hash << "#{input}#{index}"
end


hash2 = Digest::MD5.new
index2 = 0

while hash2.hexdigest[0..5] != '000000'
  hash2.reset
  index2 += 1
  p index2 if index2 % 1000 == 0
  hash2 << "#{input}#{index2}"
end

puts "Part 1: #{index}"
puts "Part 2: #{index2}"