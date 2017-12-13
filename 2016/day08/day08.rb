# http://adventofcode.com/2016/day/8

class Pixel
    attr_accessor :x, :y, :value
    def initialize(x=0, y=0, val=false)
        @x = x + 0
        @y = y + 0
        @value = val

        # print()
    end
    def shift_x( mag, mod )
        x = @x + mag
        @x = x % mod
    end
    def shift_y( mag, mod )
        x = @y + mag
        @y = x % mod
    end
    def toggle()
        @value = true
    end

    def print()
        puts "Pixel: x: #{@x}, y: #{@y}, val: #{@value}"
    end
end

class Display

    def initialize(width, height)
        @width = width
        @height = height
        @pixels = build_display(@width, @height)
    end
    def build_display(width, height)
        arr = []
        for x in 0...width
            for y in 0...height
                pixel = Pixel.new(x, y)
                arr.push(pixel)
            end
        end
        arr
    end
    def parse_instructions( arr )
        arr.each { |i| 
            i_set = i.split(' ')
            type = i_set.shift()

            case type 
                when "rect"
                    # puts "rect"
                    x, y = i_set[0].split('x')
                    square(x.to_i, y.to_i)
                when "rotate"
                    axis = i_set[0]
                    index = i_set[1].slice(/\d+/).to_i
                    mag = i_set[3].to_i
                    # puts "#{axis}, #{index}, #{mag}"
                    rotate(axis, index, mag)
                else 
                    puts "Err"
                end
            print()
            puts ""
         }
    end
    def rotate(axis, index, mag)
        case axis
            when "column"
                # puts "shift col"
                shift_col(index, mag)
            when "row"
                # puts "shift row"
                shift_row(index, mag)
            else
                puts "Err"
        end
    end
    def shift_row(row_index, magnitude)
        @pixels
            .each { |p| if p.y == row_index then 
                p.shift_x(magnitude, @width)
            end }
    end

    def shift_col(col_index, magnitude)
        @pixels
            .each { |p| if p.x == col_index then 
                p.shift_y(magnitude, @height)
            end }
    end
    def sort()
        @pixels.sort! { |a, b| 
            if a.y > b.y then 
                1
            elsif a.y < b.y then 
                -1
            elsif a.x > b.x then
                1
            else
                -1
            end
        }
    end
    def square(x, y)
         @pixels.each { |p| 
            if p.x < x && p.y < y then
                p.toggle()
            end
         }
    end
    def count()
        @pixels.select { |p| p.value }.length
    end
    def get_row(index)
        @pixels.select { |p| p.y == index }
            .sort { |a,b| a.x <=> b.x }
            .map { |p| p.value }
            .map { |p| p ? '#' : "." }
            .join('')
    end
    def print()
        for i in 0...@height
            puts get_row(i)
        end
    end     
end


inst_arr = File.readlines('_data.txt')
d = Display.new(50,6)
d.parse_instructions(inst_arr)
puts "Lit Pixels: #{d.count()}"

# not 57 (too low)
# not 65 (too low)