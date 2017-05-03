class Instruction
    attr_accessor :type
    def initialize(str)
        @type, *@params = str.split()
    end
    def mutate(str)
        p = parse_params(@type, str)
        case type 
        when "rotate"
            rotate(str, p)
        when "swap"
            swap(str, p)
        when "reverse"
            reverse(str, p)
        when "move"
            move(str, p)
        else
        throw "Irregular instruction type!"
        end
    end
    def rotate(str, params)
        # puts [str, params]
        if params[0] == "right"
            tail= str.slice!(0 - params[1], params[1])
            tail << str
        else
            head = str.slice!(0, params[1])
            str << head
        end
    end
    def swap(str, params)
        str
    end
    def reverse(str, params)
        #[str, params]
        str
    end
    def move(str, params)
        str
    end
    def parse_params(type, str)
        case type 
        when "rotate"
            if @params[0] == "right" || @params[0] == "left"
            # rotate right/left 0 steps
                dir = @params[0]
                amount = @params[1].to_i
            else
            # rotate based on position of letter g
                dir = "right"
                amount = str.index(@params[-1])
            end
            [dir, amount]
        when "swap"
            # swap position 4 with position 1
        when "reverse"
            # reverse positions 2 through 3
            [@params[1], @params[3]]
        when "move"
            
        else
        throw "Irregular instruction type!"
        end    
    end
end

class Password
    def initialize(str, instrs)
        @pwd = str
        @instrs = instrs
    end
    def scramble
        for inst in @instrs
            puts "#{@pwd}"
            @pwd = inst.mutate(@pwd)
        end
        @pwd
    end
end


input = File.readlines('input.txt')
    .map { |l| Instruction.new(l) }

Password.new("abcdefgh", input).scramble