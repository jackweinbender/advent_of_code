
def dragonize(string, length)
    # puts string
    string if length == string.length
    if length <  string.length
        string[0, length]
    else
        reversed = string.reverse
        string << "0"
        string << reversed.chars.map { |c| c == "0" ? "1" : "0" }.join('')
        dragonize(string, length)
    end
end

def get_checksum(string)
    while string.length % 2 == 0
        # puts string
        string = string.scan(/.{2}/).map do |pair|
            pair[0] == pair[1] ? "1" : "0"
        end.join
    end
    string
end

# test = dragonize("10000", 20)
# puts "Test passed? #{get_checksum(test) == "01100"}"

# Answer #1: 11100110111101110
# Answer #2: 10001101010000101
input = "10111100110001111"
length = 35651584

data = dragonize(input, length)
puts "Checksum: #{get_checksum(data)}"