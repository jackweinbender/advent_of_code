require 'digest'

class CodeGen
    def decode(str)
        code = []
        index = 0
        while code.length < 8 do
            md5 = Digest::MD5.hexdigest str + index.to_s
            *head, digit = md5.split("").shift(6)
            
            if head.join("") == "00000" then
                code.push(digit)
            end

            index += 1

        end
        
        return code.join("")

    end
end


# input = "abc"
input = "uqwqemis"
gen = CodeGen.new()

puts "Door with id: '#{input}' use code '#{gen.decode(input)}'"
# puts "Expected: 18f47a30"