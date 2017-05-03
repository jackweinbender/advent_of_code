# 0 - 4294967295

cieling = 4294967295

blacklist = File.readlines('input.txt')
    .map { |b| b.split('-').map(&:to_i) }
    .sort { |a,b| a[0] <=> b[0] }

fr_max = 0
for bl in blacklist
    if bl[0] <= fr_max
        fr_max = [bl[1], fr_max].max
        next
    end
    puts "First IP: #{fr_max+1}"
    break
end

# 17348574