

class Instr
    attr_accessor :type, :instructions
    def initialize(str)
        @type, *@instructions = str.split()
    end
    def stringify()
        "#{@type}: [#{@instructions}]"
    end
end


class Register
    def initialize( instructions, register_state)
        @instructions = instructions
            .map { |i| Instr.new(i) }
        @register = register_state
        @output = ""
    end

    def execute
        idx = 0
        while idx < @instructions.length && @output.length < 10 do
            # puts "#{idx}: #{@instructions[idx].stringify()}"
            
            ##############################
            #### Manual Optimisations ####
            ###############################
            if idx == 2 
                @register[:d] = @register[:a] + (11 * 231)
                @register[:b] = 0
                @register[:c] = 0
                idx = 8
            end
            if idx == 12
                @register[:a] = @register[:a] + (@register[:b] / 2)
                @register[:c] = -1 * (@register[:b] % 2).abs + 2
                @register[:b] = 0
                idx = 20
            end
            if idx == 21
                @register[:b] = @register[:b] - @register[:c]
                @register[:c] = 0
                idx = 26
            end

            ###############################
            ###############################
            ###############################
            case @instructions[idx].type
            when 'jnz'
                idx = jump(idx, @instructions[idx].instructions)
                # puts "jump to index #{idx}: #{@register}"
            when 'inc'
                increment(@instructions[idx].instructions[0])
                idx = idx + 1
            when 'dec'
                decrement(@instructions[idx].instructions[0])
                idx = idx + 1
            when "cpy"
                copy(@instructions[idx].instructions)
                idx = idx + 1
            when "out"
                # puts @register[:b]
                @output = "#{@output}#{@register[:b]}"
                idx = idx + 1
            else
                throw "Irregular type!"
            end
            # puts idx
        end
        
        [@output, @register]
    end
    def increment(reg)
        @register[reg.to_sym] += 1
        #puts "inc #{reg}: #{@register}"
    end
    def decrement(reg)
        @register[reg.to_sym] -= 1
        #puts "dec #{reg}: #{@register}"
    end
    def copy(instr)
        if instr[0].match(/[0-9]/)
            @register[instr[1].to_sym] = instr[0].to_i
        elsif instr[0].match(/[0-9]/) && instr[1].match(/[0-9]/)
        else
            @register[instr[1].to_sym] = @register[instr[0].to_sym]
        end
        #puts "copy #{instr[0]} to #{instr[1]}: #{@register}"
    end
    def jump(curr_index, instr)
        case instr[0]
        when "a","b","c","d"
            if @register[instr[0].to_sym] == 0
                curr_index + 1
            else
                curr_index + instr[1].to_i
            end
        when "0"
            curr_index + 1
        else
            curr_index + instr[1].to_i
        end
    end
end


input = File.readlines('input.txt')

for i in (1..1000)
    register = {
        :a => i,
        :b => 0,
        :c => 0,
        :d => 0
    }
    
    out = Register.new(input, register).execute()
    if out[0] == "0101010101"
        puts "Answer #{out[0]} at #{i}"
        break
    end
end