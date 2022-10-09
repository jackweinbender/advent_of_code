input = File.read('input.txt').split('')

numbers = input.map { |x| x == '(' ? 1 : -1 }
puts "Part 1: #{numbers.sum}"

floor = 0

numbers.each_with_index do |adj, idx|
  if floor.negative?
    puts "Part 2: #{idx}"
    break
  else
    floor += adj
  end
end