

class Instr
    attr_accessor :type, :instructions
    def initialize(str)
        @type, *@instructions = str.split()
    end
    def toggle()
        case @instructions.length
        when 1
            if @type == "inc"
                @type = "dec"
            else
                @type = "inc"
            end
        else
            if @type == "jnz"
                @type = "cpy"
            else
                @type = "jnz"
            end
        end
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
    end

    def execute
        idx = 0
        while idx < @instructions.length do
            # puts "#{idx}: #{@instructions[idx].stringify()}"
            
            ##############################
            #### Manual Optimisations ####
            ###############################
            if idx == 5 && @register[:d] > 0
                @register[:a] += ( @register[:c] * @register[:d] )
                @register[:c] = 0
                @register[:d] = 0
                idx = 10
            end
            if idx == 21
                @register[:a] += @register[:d]
                @register[:d] = 0
                idx = 24
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
            when "tgl"
                reg = @instructions[idx].instructions[0].to_sym
                idx_to_toggle = idx + @register[reg]

                if idx_to_toggle < @instructions.length
                    @instructions[idx_to_toggle].toggle()
                end
                idx = idx + 1
            else
                throw "Irregular type!"
            end
            # puts idx
        end
        
        @register
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
        if @register[instr[0].to_sym] == 0
            curr_index + 1
        elsif instr[1].match(/[0-9]/)
            instr[1].to_i + curr_index
        else
            @register[instr[1].to_sym].to_i + curr_index
        end
    end
    def to_string
        #puts "#{@register}"
    end
end

test_input = File.readlines('test_input.txt')
test_register = {
    :a => 0,
    :b => 0,
    :c => 0,
    :d => 0
}
input = File.readlines('input.txt')
register = {
    :a => 12,
    :b => 0,
    :c => 0,
    :d => 0
}
# Register.new(test_input, test_register).execute()
puts Register.new(input, register).execute()

# Answer #1 {:a=>12480, :b=>1, :c=>0, :d=>0}
# Answer #2 {:a=>479009040, :b=>1, :c=>0, :d=>0}