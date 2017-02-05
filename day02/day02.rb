
pad = {
    1 => {l: false, r: 2, u: false, d: 4},
    2 => {l: 1, r: 3, u: false, d: 5},
    3 => {l: 2, r: false, u: false, d: 6},
    4 => {l: false, r: 5, u: 1, d: 7},
    5 => {l: 4, r: 6, u: 2, d: 8},
    6 => {l: 5, r: false, u: 3, d: 9},
    7 => {l: false, r: 8, u: 4, d: false},
    8 => {l: 7, r: 9, u: 5, d: false},
    9 => {l: 8, r: false, u: 6, d: false}
}

new_pad = {
    1 => {l: false, r: false, u: false, d: 3},
    2 => {l: false, r: 3, u: false, d: 6},
    3 => {l: 2, r: 4, u: 1, d: 7},
    4 => {l: 3, r: false, u: false, d: 8},
    5 => {l: false, r: 6, u: false, d: false},
    6 => {l: 5, r: 7, u: 2, d: :A},
    7 => {l: 6, r: 8, u: 3, d: :B},
    8 => {l: 7, r: 9, u: 4, d: :C},
    9 => {l: 8, r: false, u: false, d: false},
    :A => {l: false, r: :B, u: 6, d: false},
    :B => {l: :A, r: :C, u: 7, d: :D},
    :C => {l: :B, r: false, u: 8, d: false},
    :D => {l: false, r: false, u: :B, d: false}
}

class Keypad 
    def initialize (pad, start_pos=5)
        @pad = pad
        @position = start_pos 
        @code = []
    end
    
    def get_position()
        return @position
    end

    def get_code()
        return @code.join("")
    end

    def read_line(line)
        for move in line.split("")
            #puts "pos: #{@position}, move: #{move}"
            move(@position, move)
        end
        @code.push(@position)
    end
    
    def move (pos, move)
        
        next_pos = @pad[@position][move.downcase.to_sym]

        if next_pos then
            puts "from #{pos} #{move} to #{next_pos}"
            @position =  next_pos
        else
            puts "can't move #{move} from #{pos}"
        end

    end
end

input = File.readlines("_data.txt")

keypad = Keypad.new( new_pad , 5 )

for line in input
    keypad.read_line(line.strip)
end

puts "The code is: #{keypad.get_code()}"