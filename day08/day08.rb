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


            puts "set: #{i_set} || type: #{type}"

         }
    end
    def shift_row(row_index, magnitude)
        @pixels
            .each { |p| if p.y == row_index then 
                p.shift_x(magnitude, @width)
            end }
            .sort! { |a, b| a.y <=> b.y  }
            .sort! { |a, b| a.x <=> b.x  }
    end

    def shift_col(col_index, magnitude)
        @pixels
            .each { |p| if p.x == col_index then 
                p.shift_y(magnitude, @height)
            end }
            .sort! { |a, b| a.y <=> b.y  }
            .sort! { |a, b| a.x <=> b.x  }

    end
    def square(x, y)
         @pixels.each { |p| 
            if p.x < x && p.y < y then
                p.toggle()
            end
         }
    end
    def toggle()
        @pixels[0].toggle()
    end
    def print()
       @pixels.each { |p| p.print() }
    end
    
        
end

w = 7
h = 3

inst_arr = File.readlines('_test_data.txt')
d = Display.new(w, h)
d.parse_instructions(inst_arr)





# d.print()