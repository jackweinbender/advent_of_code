class Room
    def initialize ( str )
        @name = str.split("[")[0]
        @checksum = str[/.*(\[.*\])/, 1]
        puts "Initialized: #{@name}, cksm: #{@checksum}"
    end
end


input = File.readlines("_test_data.txt").strip

for line in input
    Room.new(line)
end
