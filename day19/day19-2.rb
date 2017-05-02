
num_elfs = 3005290
left_queue = []
right_queue = []

puts "Building circle...\n"
for i in 1..num_elfs
    if i <= num_elfs / 2
        left_queue.push(i.to_s)
    else
        right_queue.push(i.to_s)
    end
end

puts "Reducing Loop...\n"
while left_queue.length > 0 && right_queue.length > 0
    print "."
    a = left_queue.shift()
    b = right_queue.shift()
    # puts "#{a}, #{b}"
    right_queue.push(a)
    
    if right_queue.length - left_queue.length >= 2
        c = right_queue.shift()
        left_queue.push(c)
    end
end

puts "\nWinner: #{right_queue.pop()}"