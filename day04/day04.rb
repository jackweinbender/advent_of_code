class Room
    def initialize ( str )
        *@name, @sector = str.split("[")[0].strip.split("-")
        @checksum = str[/.*\[(.*)\]/, 1].strip
        puts "INITIALIZED:\n- name: '#{@name}'\n- cksm: '#{@checksum}'\n- id: '#{@sector}'"
    end
    def summarize()
        name_normalized = @name.join("")
    end
    
end


input = File.readlines("_test_data_1514.txt")

for line in input
    Room.new(line)
end
