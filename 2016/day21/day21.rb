class Instruction
    attr_accessor :type
    def initialize(str)
        @type, *@params = str.split()
    end
    def mutate(str)
        p = parse_params(@type, str)
        case @type 
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
    def unmutate(str)
        p = unparse_params(@type, str)
        case @type 
        when "rotate"
            unrotate(str, p)
        when "swap"
            swap(str, p)
        when "reverse"
            reverse(str, p)
        when "move"
            unmove(str, p)
        else
        throw "Irregular instruction type!"
        end
    end
    def rotate(str, params)
        if params[0] == "right"
            tail= str.slice!(0 - params[1], params[1])
            tail << str
        else
            head = str.slice!(0, params[1])
            str << head
        end
    end
    def unrotate(str, params)
        if params[0] == "right"
            params[0] = "left"
        else
            params[0] = "right"
        end
        rotate(str, params)
    end
    def swap(str, params)
        if params[0] == 'pos'
            ind_x = params[1]
            ind_y = params[2]

            x = str[ind_x]
            y = str[ind_y]
        else
            ind_x = str.index(params[1])
            ind_y = str.index(params[2])

            x = params[1]
            y = params[2]
        end
        str[ind_x] = y
        str[ind_y] = x

        str
    end
    def reverse(str, params)
        s = params[0]
        e = params[1]

        pre     = str[0...s]
        rev     = str[s..e].reverse
        post    = str[e+1..-1]
        
        pre << rev << post
    end
    def move(str, params)
        ch = str[params[0]]
        str.delete! ch
        str.insert(params[1], ch)
        str
    end
    def unmove(str, params)
        ch = str[params[1]]
        str.delete! ch
        str.insert(params[0], ch)
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
                amount = str.index(@params[-1]) + 1

                if amount > 4
                    amount = amount + 1
                end
            end
            [dir, amount % str.length ]
        when "swap"
            if @params[0] == 'position'
                t = 'pos'
                x = @params[1].to_i
                y = @params[4].to_i
            else
                t = 'letter'
                x = @params[1]
                y = @params[4]
            end
            [t, x, y]
        when "reverse"
            # reverse positions 2 through 3
            [@params[1].to_i, @params[3].to_i]
        when "move"
            # move position 1 to position 4
            [@params[1].to_i, @params[4].to_i]
        else
        throw "Irregular instruction type!"
        end    
    end
    def unparse_params(type, str)
        case type 
        when "rotate"
            if @params[0] == "right" || @params[0] == "left"
            # rotate right/left 0 steps
                dir = @params[0]
                amount = @params[1].to_i % str.length
            else
            # rotate based on position of letter g
                dir = "right"
                idx = str.index(@params[-1])

                if idx % 2 == 0
                    orig_idx = (idx + str.length - 2) / 2
                    amount = orig_idx - idx
                else
                    orig_idx = (idx - 1) / 2
                    amount = idx - orig_idx
                end
                # amount = idx - orig_idx
            end
            [dir, amount]
        when "swap"
            if @params[0] == 'position'
                t = 'pos'
                x = @params[1].to_i
                y = @params[4].to_i
            else
                t = 'letter'
                x = @params[1]
                y = @params[4]
            end
            [t, x, y]
        when "reverse"
            # reverse positions 2 through 3
            [@params[1].to_i, @params[3].to_i]
        when "move"
            # move position 1 to position 4
            [@params[1].to_i, @params[4].to_i]
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
            print "#{@pwd} >> "
            @pwd = inst.mutate(@pwd)
            puts "#{@pwd}"
        end
        @pwd
    end
    def unscramble
        for inst in @instrs.reverse
            print "#{@pwd} >> "
            @pwd = inst.unmutate(@pwd)
            puts "#{@pwd}"
        end
        @pwd
    end
    def get_pwd
        @pwd
    end
end

test_input = File.readlines('test_input.txt')
    .map {|l| Instruction.new(l)}


test = Password.new("abcde", test_input).scramble

print "Test Passing: "
puts 'decab' == test


input = File.readlines('input.txt')
    .map { |l| Instruction.new(l) }


puts  "Answer #1: " << Password.new("abcdefgh", input).scramble

test2 = Password.new("decab", test_input).unscramble

print "Test #2 Passing: "
puts 'abcde' == test2

puts  "Answer #2: " << Password.new("fbgdceah", input).unscramble