

class Instr
    attr_accessor :type, :instructions
    def initialize(str)
        @type, *@instructions = str.split()
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
            case @instructions[idx].type
            when 'jnz'
                idx = jump(idx, @instructions[idx].instructions)
                #puts "jump to index #{idx}: #{@register}"
            when 'inc'
                increment(@instructions[idx].instructions[0])
                idx = idx + 1
            when 'dec'
                decrement(@instructions[idx].instructions[0])
                idx = idx + 1
            when "cpy"
                copy(@instructions[idx].instructions)
                idx = idx + 1
            else
                throw "Irregular type!"
            end
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
        else
            @register[instr[1].to_sym] = @register[instr[0].to_sym]
        end
        #puts "copy #{instr[0]} to #{instr[1]}: #{@register}"
    end
    def jump(curr_index, instr)
        if @register[instr[0].to_sym] == 0
            curr_index + 1
        else
            instr[1].to_i + curr_index
        end
    end
    def to_string
        #puts "#{@register}"
    end
end

# Answer 1 :a=>318007
# Answer 2 :a=>9227661
input = File.readlines('_data.txt')
register = {
    :a => 0,
    :b => 0,
    :c => 1,
    :d => 0
}

puts Register.new(input, register).execute()