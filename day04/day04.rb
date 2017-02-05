class Room
    def initialize ( str )
        *@name, @sector = str.split("[")[0].strip.split("-")
        @checksum = str[/.*\[(.*)\]/, 1].strip
        #puts "INITIALIZED:\n- name: '#{@name}'\n- cksm: '#{@checksum}'\n- id: '#{@sector}'"
    end
    def get_sector()
        @sector.to_i
    end
    
    def summarize()
        name_normalized = @name.join("").split("").sort
        name_hash = name_normalized
            .reduce({}) do |acc, name|
                acc.merge({name => 1}){|key, oldval, newval| newval + oldval}
            end
        
        Hash[name_hash.sort_by{|k, v| [v*-1, k]}].keys.shift(5).join("")
    end
    def is_valid()
        if summarize() == @checksum then
            puts "#{summarize()} == #{@checksum}"
            true
        else
            puts "#{summarize()} != #{@checksum}"
            false
        end
        
    end
end


# total = File.readlines("_test_data_1514.txt")
total = File.readlines("_data.txt")
    .map { |line| Room.new(line) }
    .select { |room| room.is_valid() }
    .reduce(0) {|acc, room| acc + room.get_sector()}

puts "Total: #{total}"