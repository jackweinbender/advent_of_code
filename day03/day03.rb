class Triangle
    def initialize ( triple )
        @x = triple[0].to_i
        @y = triple[1].to_i
        @z = triple[2].to_i
    end
    def is_valid()
        @x + @y > @z and
        @x + @z > @y and
        @z + @y > @x
    end
end
    

class InputData
    def initialize ( arr )
        @rows = arr.map {|line| line.strip.split(" ")}
        @lines = arr.map {|line| line.strip.split(" ")}
        @cols = to_cols(arr)
        @pointer = 0
    end
    def to_cols( arr )
        lines = @lines
        columns = []

        while lines.length > 0
            cols = rotate(lines.shift(3))
            columns.push(cols)
        end
        return columns.flatten(1)
    end
    def get_cols()
        return @cols
    end
    def get_rows()
        return @rows
    end
    def rotate( arr )
        topos = arr.reduce([[], [], []]) do |acc, line|
            acc[0].push(line[0])
            acc[1].push(line[1])
            acc[2].push(line[2])
            acc
        end 
        return topos
    end
    
    
end

lines = File.readlines("_data.txt")

data_col = InputData.new(lines).get_cols()
data_row = InputData.new(lines).get_rows()

puts "#{data_row}"

by_row = data_row
    .select{ |r| Triangle.new(r).is_valid() }
    .length
by_col = data_col
    .select{ |r| Triangle.new(r).is_valid() }
    .length

puts "Valid Triangles (rows): #{by_row}"
puts "Valid Triangles (cols): #{by_col}"