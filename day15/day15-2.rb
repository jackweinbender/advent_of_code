class Disc
    attr_accessor :v_offset, :size, :r_pos
    def initialize(arr)
        @v_offset, @size, @r_pos = arr
    end
    def to_zero
        @size - @r_pos
    end
    def aligned?(drop_at)
        state = drop_at + @r_pos + @v_offset
        if state < @size 
            false
        elsif state % @size == 0
            true
        else
            false
        end
    end
end

input = File.readlines('input-2.txt')
    .map { |line| 
        _, *disc = line
            .match(/(\d+).* (\d+) .*time=0.* (\d+)/).to_a
            .map(&:to_i)
        Disc.new(disc) }
input.each do |d|
    puts "#{d}"
end

time = 0
until input.all? { |d| d.aligned?(time) }
    step = 1
    input.select { |d| d.aligned?(time) }
        .each do |d|
            step *= d.size
        end
    puts "Coef: #{step}"
    time += step
end

# Wait for 3208099 seconds.
puts "Wait for #{time} seconds."
