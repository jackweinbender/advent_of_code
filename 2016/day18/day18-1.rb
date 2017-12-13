class Line
    attr_accessor :line
    def initialize(str)
        @line = str
    end
    def next
        Line.new(to_new_line(@line))
    end
    def num_safe
        @line.chars.select{ |c| c == "." }.count
    end
    def to_new_line(line)
        new_line = ""
        for i in 0..line.length - 1
            if i == 0
                line[i+1] == "^" ? new_line << "^" : new_line << "."
            elsif i+1 >= line.length || line[i+1] == "."
                line[i-1] == "^" ? new_line << "^" : new_line << "."
            else
                line[i-1] != line[i+1] ? new_line << "^" : new_line << "."
            end
        end
        new_line
    end
end

# input = ["..^^.", 3]
# input = [".^^.^.^^^^", 10]
input = ["^^^^......^...^..^....^^^.^^^.^.^^^^^^..^...^^...^^^.^^....^..^^^.^.^^...^.^...^^.^^^.^^^^.^^.^..^.^", 400000]

line = Line.new(input[0])
total_safe = 0

(input[1]).times do
    # puts line.line
    print "."
    total_safe += line.num_safe
    line = line.next()
end

puts "There are #{total_safe} safe tiles."