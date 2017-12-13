require 'digest'

class CodeGen

    def decode(str, len)
        code = Array.new(len, false)
        i = 0
        while code.include? false do
            md5 = Digest::MD5.hexdigest str + i.to_s
            *head, index, digit = md5.split("").shift(7)
            #puts "#{code} :: #{head}, #{index}, #{digit} --- i:#{i}"
            if head.join("") == "00000" then
                
                if ("0"..(len-1).to_s).to_a.include? index then
                
                    if code[index.to_i] == false then
                        puts "Iter: #{i.to_i}, Index: #{index}, Digit: #{digit}"
                        code[index.to_i] = digit
                    end
                end

            end

            i += 1

        end
        
        return code.join("")

    end
end

# input = "abc"
input = "uqwqemis"
gen = CodeGen.new()

puts "Door with id: '#{input}' use code '#{gen.decode(input, 8)}'"
# puts "Expected: 18f47a30" # Original 
# puts "Expected: 05ace8e3"