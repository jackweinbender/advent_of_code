#! /usr/bin/env ruby


input = $stdin.readlines(chomp: true)

position = 50
zero_stops = 0
zero_passes = 0

input.each do |instr|
  dir, *amt = instr.split("")
  amount = amt.join("").to_i

  mag = amount.div(100)
  additional_zeroes = 0

  additional_zeroes += mag
  starting_pos = position

  if dir == "L"
    position = (starting_pos - amount) % 100
    additional_zeroes += 1 if (position > starting_pos && starting_pos != 0) || position == 0
  elsif dir == "R"
    position = (starting_pos + amount) % 100
    additional_zeroes += 1 if (position < starting_pos && starting_pos != 0) || position == 0
  else
    throw StandardError
  end
  zero_passes += additional_zeroes
  zero_stops += 1 if position == 0
end




part_one = zero_stops
part_two = zero_passes

puts "Part 1: #{part_one}"
puts "Part 2: #{part_two}"
