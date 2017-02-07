
code = Array.new(8, {})

input = File.readlines("_test_data_easter.txt")
input = File.readlines("_data.txt")
    .map { |line| line.strip }
    .map { |line| line.split("").map { |ch| { ch => 1 } } }
    .reduce(code) {
        |acc, line|
        #puts "#{acc} || #{line}"
        for i in 0..7
            acc[i] = acc[i].merge(line[i]) { |k, o, n| o + n }
            #puts "#{acc}"
        end
        acc
    }.map { |hash| 
        # Part 1
        hash.max_by { |k, v| v } 
        # Part 2
        # hash.min_by { |k, v| v } 
    }
    .map { |hash| hash[0] }
    .join("")

puts "#{input}"

# qtbjqi