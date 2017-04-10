

class Parser
    def initialize()
    end
    def parse( str )
        output = ''
        i = 0
        while i < str.length
            case str[i]
            when "("
                match = str.match(/\d+x\d+/, i+1)[0]
                len, mag = match.split("x").map(&:to_i)
                chunk_index = i + match.length + 2

                chunk = str.slice( chunk_index, len)

                mag.times { output << chunk }

                i = chunk_index + len
            else
                output << str[i]
                i += 1
            end
        end
        output
    end
    def parse_v2_len( str )
        output = 0
        i = 0
        while i < str.length
            case str[i]
            when "("
                match = str.match(/\d+x\d+/, i+1)[0]
                len, mag = match.split("x").map(&:to_i)
                chunk_index = i + match.length + 2

                chunk_len = parse_v2_len(str.slice( chunk_index, len))
                # puts chunk_len
                mag.times { output += chunk_len }

                i = chunk_index + len
            else
                output += 1
                i += 1
            end
        end
        output
    end
end


## V1
str = File.readlines('_data.txt')
    .map{ |l| l.chomp }.join('')
p = Parser.new()
puts "V1: #{p.parse( str ).length}"

# test_data = [
#     ["ADVENT", 6],
#     ["A(1x5)BC", 7]
# ]

# p = Parser.new()
# test_data.each do |test|
#     puts p.parse( test[0] ).length == test[1]
# end

## V2
str = File.readlines('_data.txt')
    .map{ |l| l.chomp }.join('')
p = Parser.new()
puts "V2: #{p.parse_v2_len( str )}"

# test_data = [
#     ["(3x3)XYZ", 9],
#     ["X(8x2)(3x3)ABCY", 20],
#     ["(27x12)(20x12)(13x14)(7x10)(1x12)A", 241920],
#     ["(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN", 445]
# ]

# p = Parser.new()
# test_data.each do |test|
#     puts p.parse_v2_len( test[0] ) == test[1]
# end