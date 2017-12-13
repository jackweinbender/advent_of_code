class Grid
    def initialize( input )
        @instructions = input.split(",").map { |n| n.strip }
        @position = [0,0]
        @direction = 0
        @stops = []
        @overlaps = []
    end
    def first_overlap()
        fo = @overlaps[0]
        puts "First overlap: #{fo} | Dist: #{(fo[0] + fo[1]).abs}"
    end
    
    def navigate()
        puts "Starting at #{get_position()}\n--------\n"
        for instr in @instructions

            set_direction(instr[0])
            
            turn = get_direction()
            mag = instr[1,instr.length]

            move( turn, mag )
            
        end
        
        pos = get_position()

        puts "--------\nEnded at #{pos}"
        puts "Total Distance: #{(pos[0] + pos[1]).abs}"
        

    end
    
    def get_position()
        return @position
    end
    
    def get_direction()
        case @direction % 4
        when 0 # North
            return [0,1]
        when 1 # East
            return [1,0]
        when 2 # South
            return [0,-1]
        when 3 # West
            return [-1,0]
        end
    end
    def set_direction( turn )
        case turn[0]
        when "R"
            @direction += 1
        when "L"
            @direction -= 1
        end
    end
    
    def move( turn, mag )

        for i in 1..mag.to_i
            new_pos = [@position[0] + turn[0], @position[1] + turn[1]]
            
            if @stops.include? new_pos.join(",")
                @overlaps.push(new_pos)
                @stops.push(new_pos.join(","))
                @position = new_pos
                puts "Been here: #{@position}"
            else
                @position = new_pos
                @stops.push(new_pos.join(","))
                puts "Never been here: #{@position}"
            end
        end

    end
end

input = File.read("_data.txt")

grid = Grid.new( input )

grid.navigate()
grid.first_overlap()