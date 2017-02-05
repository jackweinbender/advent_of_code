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
            # puts "#{summarize()} == #{@checksum}"
            true
        else
            # puts "#{summarize()} != #{@checksum}"
            false
        end
        
    end
    def decode()
        name = @name.map do | word |
            word.split("").map { |ch| char_shift(ch) }.join("")
        end

        name.join(" ")
    end
    def char_shift( char )
        c = ("a".."z").to_a
        mod = get_sector() % 26
        pos = c.index(char)

        index = (pos + mod) % 26
        c[index]
    end

end

puts Room.new("qzmt-zixmtkozy-ivhz-343[zimth]").decode()

valid_rooms = File.readlines("_test_data_1514.txt")
valid_rooms = File.readlines("_test_data_2.txt")
valid_rooms = File.readlines("_data.txt")
    .map { |line| Room.new(line) }
    .select { |room| room.is_valid() }

total = valid_rooms.reduce(0) {|acc, room| acc + room.get_sector()}

q = "very encrypted name"
q = "North Pole"

regex = q.downcase.split(" ").join(".*")

first_match = valid_rooms.select { |room| room.decode().match(/#{regex}/)  }.first

puts "Sector total of valid rooms: #{total}"
puts "First sector match for '#{q}' is '#{first_match.get_sector()}'"